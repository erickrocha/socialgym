mod support;

use business::domain::business_error::BusinessErrorKind as K;
use business::use_cases::account_deletion_use_case::AccountDeletionUseCase;
use business::use_cases::authentication::{Authentication, AuthenticationError, ValidateError};
use business::use_cases::logout_use_case::LogoutUseCase;
use business::use_cases::refresh_token::RefreshToken;
use business::use_cases::switch_business_profile::{
    SwitchBusinessProfile, SwitchBusinessProfileError,
};
use chrono::{Duration, Utc};
use entity::user_entity;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::time::Duration as StdDuration;
use support::{business_profile, fresh_db, kind, register};

const PASSWORD: &str = "Str0ng!Password";

fn auth_env() {
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", "uc-access-secret");
        std::env::set_var("REFRESH_TOKEN_SECRET", "uc-refresh-secret");
        std::env::set_var("AUTH_RULES_ENABLED", "true");
        std::env::set_var("LOGIN_MAX_FAILED_ATTEMPTS", "3");
        std::env::set_var("LOGIN_LOCKOUT_DURATION_SECONDS", "60");
        std::env::remove_var("TOKEN_REVOCATION_ENABLED");
        std::env::remove_var("LOGIN_LOCKOUT_ENABLED");
    }
}

/// `iat` and the revoke-all watermark have one-second resolution: wait so a fresh token is newer.
async fn tick() {
    tokio::time::sleep(StdDuration::from_millis(1100)).await;
}

async fn set_user(db: &DatabaseConnection, id: i32, f: impl FnOnce(&mut user_entity::ActiveModel)) {
    let mut am = user_entity::ActiveModel {
        id: Set(id),
        ..Default::default()
    };
    f(&mut am);
    am.update(db).await.unwrap();
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn login_validates_credentials_locks_out_and_honours_pending_deletion() {
    auth_env();
    let db = fresh_db().await;
    let user = register(&db, 1).await;
    let email = user.email.clone();
    let uid = user.id.unwrap();

    // credential checks
    assert!(matches!(
        Authentication::execute(&db, "".into(), PASSWORD.into()).await,
        Err(AuthenticationError::InvalidCredentials)
    ));
    assert!(matches!(
        Authentication::execute(&db, email.clone(), "".into()).await,
        Err(AuthenticationError::InvalidCredentials)
    ));
    assert!(matches!(
        Authentication::execute(&db, "nobody@example.test".into(), PASSWORD.into()).await,
        Err(AuthenticationError::InvalidCredentials)
    ));

    // success: both tokens, identity claims and no active business profile
    let token = Authentication::execute(&db, email.clone(), PASSWORD.into())
        .await
        .unwrap();
    assert_eq!(
        (
            token.token_type.as_str(),
            token.person_id,
            token.username.as_str()
        ),
        ("Bearer", user.person_id, email.as_str())
    );
    assert!(
        token.refresh_token.is_some()
            && token.active_business_profile_id.is_none()
            && token.pending_account_deletion.is_none()
    );
    let ctx = Authentication::validate(&db, token.access_token.clone())
        .await
        .unwrap();
    assert_eq!(
        (ctx.user.person_id, ctx.active_business_profile_id),
        (user.person_id, None)
    );

    // lockout: the third wrong password locks, and even the right password is refused afterwards
    for _ in 0..2 {
        assert!(matches!(
            Authentication::execute(&db, email.clone(), "wrong".into()).await,
            Err(AuthenticationError::InvalidCredentials)
        ));
    }
    assert!(matches!(
        Authentication::execute(&db, email.clone(), "wrong".into()).await,
        Err(AuthenticationError::InvalidCredentials)
    ));
    match Authentication::execute(&db, email.clone(), PASSWORD.into()).await {
        Err(AuthenticationError::AccountLocked {
            retry_after_seconds,
        }) => assert!(retry_after_seconds > 0 && retry_after_seconds <= 60),
        other => panic!("expected a lockout, got {other:?}"),
    }
    assert!(
        matches!(
            Authentication::validate(&db, token.access_token.clone()).await,
            Err(ValidateError::Revoked)
        ),
        "locking the account revokes tokens issued before"
    );

    // an expired lock is cleared by the next correct login
    tick().await;
    set_user(&db, uid, |u| {
        u.locked_until = Set(Some(Utc::now() - Duration::seconds(5)))
    })
    .await;
    let again = Authentication::execute(&db, email.clone(), PASSWORD.into())
        .await
        .unwrap();
    Authentication::validate(&db, again.access_token)
        .await
        .unwrap();
    let after = user_entity::Entity::find_by_id(uid)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((after.failed_login_attempts, after.locked_until), (0, None));

    // a correct login resets earlier failures
    assert!(Authentication::execute(&db, email.clone(), "wrong".into())
        .await
        .is_err());
    Authentication::execute(&db, email.clone(), PASSWORD.into())
        .await
        .unwrap();
    assert_eq!(
        user_entity::Entity::find_by_id(uid)
            .one(&db)
            .await
            .unwrap()
            .unwrap()
            .failed_login_attempts,
        0
    );

    // a disabled account cannot sign in, except while its deletion is pending (so it can be cancelled)
    set_user(&db, uid, |u| u.enabled = Set(false)).await;
    assert!(matches!(
        Authentication::execute(&db, email.clone(), PASSWORD.into()).await,
        Err(AuthenticationError::AccountDisabled)
    ));
    AccountDeletionUseCase::request_deletion(&db, uid, false)
        .await
        .unwrap();
    let pending = Authentication::execute(&db, email.clone(), PASSWORD.into())
        .await
        .unwrap();
    assert!(pending.pending_account_deletion.is_some());
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn tokens_are_validated_revoked_on_logout_and_rotated_on_refresh() {
    auth_env();
    let db = fresh_db().await;
    let user = register(&db, 1).await;
    let uid = user.id.unwrap();
    let login = || Authentication::execute(&db, user.email.clone(), PASSWORD.into());

    // validation of malformed tokens and of tokens for unknown users
    assert!(matches!(
        Authentication::validate(&db, "not.a.jwt".into()).await,
        Err(ValidateError::Invalid)
    ));
    assert!(matches!(
        Authentication::validate_refresh_token(&db, "not.a.jwt".into())
            .await
            .unwrap_err()
            .kind,
        K::Unauthorized
    ));
    assert!(matches!(
        kind(
            &RefreshToken::execute(&db, "not.a.jwt".into())
                .await
                .unwrap_err()
        ),
        K::Unauthorized
    ));

    // logout revokes the access token and its refresh token
    let first = login().await.unwrap();
    let refresh = first.refresh_token.clone().unwrap();
    let ctx = Authentication::validate(&db, first.access_token.clone())
        .await
        .unwrap();
    LogoutUseCase::execute(&db, uid, ctx.jti.clone(), ctx.exp, Some(refresh.clone()))
        .await
        .unwrap();
    assert!(matches!(
        Authentication::validate(&db, first.access_token.clone()).await,
        Err(ValidateError::Revoked)
    ));
    assert!(
        matches!(
            kind(&RefreshToken::execute(&db, refresh).await.unwrap_err()),
            K::Unauthorized
        ),
        "a revoked refresh token is refused"
    );
    LogoutUseCase::execute(
        &db,
        uid,
        "other-jti".into(),
        Utc::now().timestamp() + 60,
        Some("garbage".into()),
    )
    .await
    .unwrap();

    // refresh rotates: the new token works, the old refresh token is single-use
    tick().await;
    let second = login().await.unwrap();
    let old_refresh = second.refresh_token.clone().unwrap();
    let rotated = RefreshToken::execute(&db, old_refresh.clone())
        .await
        .unwrap();
    assert!(rotated.refresh_token.is_some());
    Authentication::validate(&db, rotated.access_token.clone())
        .await
        .unwrap();

    // replaying the rotated refresh token is treated as theft: every token of the user is revoked
    assert!(matches!(
        kind(&RefreshToken::execute(&db, old_refresh).await.unwrap_err()),
        K::Unauthorized
    ));
    assert!(matches!(
        Authentication::validate(&db, rotated.access_token).await,
        Err(ValidateError::Revoked)
    ));

    // with revocation switched off, logout is a no-op and tokens stay valid
    unsafe { std::env::set_var("TOKEN_REVOCATION_ENABLED", "false") };
    tick().await;
    let third = login().await.unwrap();
    let ctx = Authentication::validate(&db, third.access_token.clone())
        .await
        .unwrap();
    LogoutUseCase::execute(&db, uid, ctx.jti, ctx.exp, third.refresh_token.clone())
        .await
        .unwrap();
    Authentication::validate(&db, third.access_token)
        .await
        .unwrap();
    unsafe { std::env::remove_var("TOKEN_REVOCATION_ENABLED") };
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn switching_business_profile_issues_a_scoped_token_only_to_the_owner() {
    auth_env();
    let db = fresh_db().await;
    let (owner, other) = (register(&db, 1).await, register(&db, 2).await);
    let profile = business_profile(&db, &owner, "Gym").await;
    let puuid = profile.uuid.clone().unwrap();
    let login = Authentication::execute(&db, owner.email.clone(), PASSWORD.into())
        .await
        .unwrap();
    let ctx = Authentication::validate(&db, login.access_token.clone())
        .await
        .unwrap();

    assert!(matches!(
        SwitchBusinessProfile::activate(
            &db,
            &owner,
            "00000000-0000-0000-0000-00000000dead".into(),
            ctx.jti.clone(),
            ctx.exp
        )
        .await,
        Err(SwitchBusinessProfileError::NotFound)
    ));
    assert!(matches!(
        SwitchBusinessProfile::activate(&db, &other, puuid.clone(), "j".into(), ctx.exp).await,
        Err(SwitchBusinessProfileError::Forbidden)
    ));
    Authentication::validate(&db, login.access_token.clone())
        .await
        .unwrap();

    let scoped =
        SwitchBusinessProfile::activate(&db, &owner, puuid.clone(), ctx.jti.clone(), ctx.exp)
            .await
            .ok()
            .unwrap();
    assert_eq!(
        scoped.active_business_profile_uuid.as_deref(),
        Some(puuid.as_str())
    );
    let scoped_ctx = Authentication::validate(&db, scoped.access_token.clone())
        .await
        .unwrap();
    assert_eq!(scoped_ctx.active_business_profile_id, profile.id);
    assert!(
        matches!(
            Authentication::validate(&db, login.access_token).await,
            Err(ValidateError::Revoked)
        ),
        "the previous token is revoked"
    );

    let back = SwitchBusinessProfile::deactivate(&db, &owner, scoped_ctx.jti, scoped_ctx.exp).await;
    assert!(back.active_business_profile_uuid.is_none());
    assert!(matches!(
        Authentication::validate(&db, scoped.access_token).await,
        Err(ValidateError::Revoked)
    ));
}
