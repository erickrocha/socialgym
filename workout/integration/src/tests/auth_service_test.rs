//! C-010 task 3: the gRPC authentication and session service (TC-002, TC-003), through a real in-process
//! server with `GrpcAuthLayer` in front, against a disposable PostGIS database (`TEST_DATABASE_URL`).
use super::server_support::{error_key, serve, with};
use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{
    AccessToken, ActivateBusinessProfileRequest, DeactivateBusinessProfileRequest, LoginRequest, LogoutRequest,
    RefreshRequest, SignupRequest,
};
use business::commons::i18n::{ErrorKey, Locale, translate};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;
use tonic::transport::Channel;
use tonic::Code;

const PASSWORD: &str = "Str0ng!Passw0rd";

struct World {
    db: Arc<DatabaseConnection>,
    client: AuthServiceClient<Channel>,
}

async fn world(limit: u32) -> World {
    let served = serve(limit).await;
    World { db: served.db, client: AuthServiceClient::new(served.channel) }
}

fn signup(email: &str) -> SignupRequest {
    SignupRequest {
        firstname: "Ada".into(),
        surname: "Lovelace".into(),
        date_of_birth: "1990-05-05".into(),
        gender: "F".into(),
        email: email.into(),
        password: PASSWORD.into(),
        terms_version: "1.0.0".into(),
        privacy_version: "1.0.0".into(),
        terms_accepted: true,
        privacy_accepted: true,
    }
}

fn login(email: &str, password: &str) -> LoginRequest {
    LoginRequest { email: email.into(), password: password.into() }
}

async fn register(world: &mut World, email: &str) -> AccessToken {
    world.client.signup(signup(email)).await.expect("sign-up").into_inner()
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn a_person_signs_up_signs_in_refreshes_and_signs_out() {
    let mut w = world(100).await;
    let first = register(&mut w, "ada@example.test").await;
    assert!(first.refresh_token.is_some() && !first.access_token.is_empty() && first.person_id > 0, "sign-up returns tokens and the person");

    let logged = w.client.login(login("ada@example.test", PASSWORD)).await.unwrap().into_inner();
    assert_eq!(logged.person_uuid, first.person_uuid, "the same person signs in");

    let refresh = logged.refresh_token.clone().unwrap();
    let renewed = w.client.refresh(RefreshRequest { refresh_token: refresh.clone() }).await.unwrap().into_inner();
    assert!(renewed.refresh_token.is_some() && renewed.refresh_token.as_deref() != Some(refresh.as_str()), "the refresh token rotates");
    let reuse = w.client.refresh(RefreshRequest { refresh_token: refresh }).await.unwrap_err();
    assert_eq!(reuse.code(), Code::Unauthenticated, "a rotated refresh token is refused");
    // Reuse of a rotated token is treated as theft: every token issued before it stops working.
    let revoked = w.client.deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(&renewed.access_token), &[])).await.unwrap_err();
    assert_eq!((revoked.code(), revoked.message()), (Code::Unauthenticated, "token has been revoked"));

    // A new sign-in works, and sign-out revokes its access token.
    tokio::time::sleep(Duration::from_millis(1200)).await;
    let session = w.client.login(login("ada@example.test", PASSWORD)).await.unwrap().into_inner();
    let refresh_token = session.refresh_token.clone().unwrap();
    w.client.logout(with(LogoutRequest { refresh_token }, Some(&session.access_token), &[])).await.unwrap();
    let after = w.client.deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(&session.access_token), &[])).await.unwrap_err();
    assert_eq!((after.code(), after.message()), (Code::Unauthenticated, "token has been revoked"));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn sign_up_applies_the_rest_rules_with_the_rest_error_keys() {
    let mut w = world(100).await;
    let mut underage = signup("kid@example.test");
    underage.date_of_birth = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let status = w.client.signup(underage).await.unwrap_err();
    assert_eq!((status.code(), error_key(&status)), (Code::InvalidArgument, ErrorKey::UnderageRegistration.as_str()));

    let mut no_consent = signup("noconsent@example.test");
    no_consent.terms_accepted = false;
    let status = w.client.signup(no_consent).await.unwrap_err();
    assert_eq!((status.code(), error_key(&status)), (Code::InvalidArgument, ErrorKey::ConsentRequired.as_str()));

    let mut stale = signup("stale@example.test");
    stale.privacy_version = "0.0.1".into();
    assert_eq!(error_key(&w.client.signup(stale).await.unwrap_err()), "CONSENT_REQUIRED", "an outdated document version");

    let mut weak = signup("weak@example.test");
    weak.password = "short".into();
    let status = w.client.signup(weak).await.unwrap_err();
    assert_eq!((status.code(), error_key(&status)), (Code::InvalidArgument, ErrorKey::WeakPassword.as_str()));

    let mut bad_date = signup("date@example.test");
    bad_date.date_of_birth = "05/05/1990".into();
    assert_eq!(w.client.signup(bad_date).await.unwrap_err().code(), Code::InvalidArgument);

    // An email that already has an account fails the same way as any other failed sign-up.
    register(&mut w, "dup@example.test").await;
    let status = w.client.signup(signup("dup@example.test")).await.unwrap_err();
    assert_eq!((status.code(), error_key(&status)), (Code::InvalidArgument, ErrorKey::SignUpUserFailed.as_str()));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn sign_in_failures_do_not_reveal_which_emails_have_an_account() {
    let mut w = world(100).await;
    register(&mut w, "known@example.test").await;
    let wrong = w.client.login(login("known@example.test", "Wr0ng!Passw0rd")).await.unwrap_err();
    let unknown = w.client.login(login("nobody@example.test", PASSWORD)).await.unwrap_err();
    assert_eq!((wrong.code(), wrong.message(), error_key(&wrong)), (unknown.code(), unknown.message(), error_key(&unknown)));
    assert_eq!((wrong.code(), error_key(&wrong)), (Code::Unauthenticated, ErrorKey::BadCredentials.as_str()));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn repeated_failures_lock_the_account_and_the_error_is_localized() {
    let mut w = world(100).await;
    register(&mut w, "lock@example.test").await;
    let mut last = None;
    for _ in 0..business::commons::auth_config::login_max_failed_attempts() + 1 {
        last = Some(w.client.login(login("lock@example.test", "Wr0ng!Passw0rd")).await.unwrap_err());
    }
    let locked = last.unwrap();
    assert_eq!((locked.code(), error_key(&locked)), (Code::FailedPrecondition, ErrorKey::AccountLocked.as_str()));
    assert_eq!(locked.message(), translate(Locale::En, ErrorKey::AccountLocked));

    let portuguese = w.client.login(with(login("nobody@example.test", PASSWORD), None, &[("accept-language", "pt-BR,pt;q=0.9")])).await.unwrap_err();
    assert_eq!(portuguese.message(), translate(Locale::PtBr, ErrorKey::BadCredentials), "the message follows accept-language");
    assert_ne!(portuguese.message(), translate(Locale::En, ErrorKey::BadCredentials));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_public_methods_are_limited_per_address_and_the_rest_need_a_token() {
    let mut w = world(3).await;
    let from = |ip: &'static str| [("x-real-ip", ip)];
    for _ in 0..3 {
        let status = w.client.login(with(login("a@example.test", PASSWORD), None, &from("192.0.2.1"))).await.unwrap_err();
        assert_eq!(status.code(), Code::Unauthenticated, "within the allowance the call reaches the use case");
    }
    let limited = w.client.login(with(login("a@example.test", PASSWORD), None, &from("192.0.2.1"))).await.unwrap_err();
    assert_eq!(limited.code(), Code::ResourceExhausted, "the fourth call from one address is refused");
    let other = w.client.login(with(login("a@example.test", PASSWORD), None, &from("192.0.2.2"))).await.unwrap_err();
    assert_eq!(other.code(), Code::Unauthenticated, "another address is not affected");
    let refresh = (0..4).map(|_| RefreshRequest { refresh_token: "nope".into() });
    let mut last = Code::Ok;
    for request in refresh {
        last = w.client.refresh(with(request, None, &from("192.0.2.3"))).await.unwrap_err().code();
    }
    assert_eq!(last, Code::ResourceExhausted, "refresh has its own limit");

    assert_eq!(w.client.logout(LogoutRequest { refresh_token: String::new() }).await.unwrap_err().code(), Code::Unauthenticated, "logout needs a token");
    let activate = ActivateBusinessProfileRequest { business_profile_uuid: "30000000-0000-0000-0000-0000000000f1".into() };
    assert_eq!(w.client.activate_business_profile(activate).await.unwrap_err().code(), Code::Unauthenticated, "so does the profile switch");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn a_person_switches_to_an_owned_business_profile_and_back() {
    let mut w = world(100).await;
    let owner = register(&mut w, "owner@example.test").await;
    let stranger = register(&mut w, "stranger@example.test").await;
    w.db.execute_unprepared(&format!(
        "INSERT INTO business_profile (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at) VALUES
           (1, '30000000-0000-0000-0000-0000000000f1', {}, '{}', '999', 'Owner Gym', 'Professional', now(), now())",
        owner.person_id, owner.person_uuid
    ))
    .await
    .unwrap();
    let profile = "30000000-0000-0000-0000-0000000000f1";
    let activate = |token: &str, uuid: &str| with(ActivateBusinessProfileRequest { business_profile_uuid: uuid.into() }, Some(token), &[]);

    let active = w.client.activate_business_profile(activate(&owner.access_token, profile)).await.unwrap().into_inner();
    assert_eq!(active.active_business_profile_uuid.as_deref(), Some(profile), "a fresh token carries the profile");
    let old = w.client.deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(&owner.access_token), &[])).await.unwrap_err();
    assert_eq!(old.code(), Code::Unauthenticated, "the previous token was revoked by the switch");

    let back = w.client.deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(&active.access_token), &[])).await.unwrap().into_inner();
    assert!(back.active_business_profile_uuid.is_none(), "back in the personal context");

    let foreign = w.client.activate_business_profile(activate(&stranger.access_token, profile)).await.unwrap_err();
    assert_eq!((foreign.code(), error_key(&foreign)), (Code::PermissionDenied, ErrorKey::BusinessProfileForbidden.as_str()));
    let missing = w.client.activate_business_profile(activate(&stranger.access_token, "30000000-0000-0000-0000-0000000000f9")).await.unwrap_err();
    assert_eq!(missing.code(), Code::NotFound);
    let malformed = w.client.activate_business_profile(activate(&stranger.access_token, "nope")).await.unwrap_err();
    assert_eq!(malformed.code(), Code::InvalidArgument);
}
