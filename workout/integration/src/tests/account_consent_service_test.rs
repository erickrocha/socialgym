//! C-010 task 4: the gRPC consent, account-deletion and data-export services (TC-004, TC-005, TC-006),
//! through the in-process server of `server_support`, against a disposable PostGIS database.
use super::server_support::{error_key, serve, with};
use crate::proto::account::account_service_client::AccountServiceClient;
use crate::proto::account::{
    CancelAccountDeletionRequest, CreateDataExportRequest, GetDataExportRequest,
    ListDataExportsRequest, RequestAccountDeletionRequest,
};
use crate::proto::auth::auth_service_client::AuthServiceClient;
use crate::proto::auth::{
    AccessToken, DeactivateBusinessProfileRequest, LoginRequest, SignupRequest,
};
use crate::proto::consent::consent_service_client::ConsentServiceClient;
use crate::proto::consent::{
    AcceptConsentRequest, ListConsentsRequest, ListPendingConsentsRequest, RevokeConsentRequest,
};
use business::commons::i18n::ErrorKey;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;
use tonic::Code;
use tonic::transport::Channel;

const PASSWORD: &str = "Str0ng!Passw0rd";

struct World {
    db: Arc<DatabaseConnection>,
    auth: AuthServiceClient<Channel>,
    consent: ConsentServiceClient<Channel>,
    account: AccountServiceClient<Channel>,
}

async fn world() -> World {
    let served = serve(1000).await;
    World {
        db: served.db,
        auth: AuthServiceClient::new(served.channel.clone()),
        consent: ConsentServiceClient::new(served.channel.clone()),
        account: AccountServiceClient::new(served.channel),
    }
}

async fn register(w: &mut World, email: &str) -> AccessToken {
    let request = SignupRequest {
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
    };
    w.auth.signup(request).await.expect("sign-up").into_inner()
}

/// A new sign-in once the token revocation of the last second no longer covers it.
async fn sign_in_again(w: &mut World, email: &str) -> AccessToken {
    tokio::time::sleep(Duration::from_millis(1200)).await;
    w.auth
        .login(LoginRequest {
            email: email.into(),
            password: PASSWORD.into(),
        })
        .await
        .expect("sign-in")
        .into_inner()
}

fn accept(document: &str, version: &str, accepted: bool) -> AcceptConsentRequest {
    AcceptConsentRequest {
        document: document.into(),
        version: version.into(),
        accepted,
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn consent_is_listed_accepted_and_revoked_with_the_rest_rules() {
    let mut w = world().await;
    let ada = register(&mut w, "ada@example.test").await;
    let token = ada.access_token.as_str();

    let listed = w
        .consent
        .list_consents(with(ListConsentsRequest {}, Some(token), &[]))
        .await
        .unwrap()
        .into_inner();
    let mut documents: Vec<_> = listed
        .consents
        .iter()
        .map(|c| c.document.as_str())
        .collect();
    documents.sort();
    assert_eq!(
        documents,
        ["privacy", "terms"],
        "sign-up accepted the two mandatory documents"
    );
    let pending = w
        .consent
        .list_pending_consents(with(ListPendingConsentsRequest {}, Some(token), &[]))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        pending
            .pending
            .iter()
            .map(|p| p.document.as_str())
            .collect::<Vec<_>>(),
        ["health_data"]
    );

    let accepted = w
        .consent
        .accept_consent(with(accept("health_data", "1.0.0", true), Some(token), &[]))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        (accepted.document.as_str(), accepted.revoked_at.is_none()),
        ("health_data", true)
    );
    let pending = w
        .consent
        .list_pending_consents(with(ListPendingConsentsRequest {}, Some(token), &[]))
        .await
        .unwrap()
        .into_inner();
    assert!(pending.pending.is_empty(), "nothing is left to accept");

    let again = w
        .consent
        .accept_consent(with(accept("health_data", "1.0.0", true), Some(token), &[]))
        .await
        .unwrap_err();
    assert_eq!(
        again.code(),
        Code::AlreadyExists,
        "an active consent is not accepted twice"
    );
    let declined = w
        .consent
        .accept_consent(with(
            accept("health_data", "1.0.0", false),
            Some(token),
            &[],
        ))
        .await
        .unwrap_err();
    assert_eq!(
        (declined.code(), error_key(&declined)),
        (Code::InvalidArgument, ErrorKey::ConsentRequired.as_str())
    );
    let outdated = w
        .consent
        .accept_consent(with(accept("health_data", "9.9.9", true), Some(token), &[]))
        .await
        .unwrap_err();
    assert_eq!(
        (outdated.code(), error_key(&outdated)),
        (Code::InvalidArgument, ErrorKey::ConsentRequired.as_str())
    );

    w.consent
        .revoke_consent(with(
            RevokeConsentRequest {
                document: "health_data".into(),
            },
            Some(token),
            &[],
        ))
        .await
        .unwrap();
    let twice = w
        .consent
        .revoke_consent(with(
            RevokeConsentRequest {
                document: "health_data".into(),
            },
            Some(token),
            &[],
        ))
        .await
        .unwrap_err();
    assert_eq!(
        twice.code(),
        Code::NotFound,
        "no active consent is left to revoke"
    );
    let unknown = w
        .consent
        .revoke_consent(with(
            RevokeConsentRequest {
                document: "marketing".into(),
            },
            Some(token),
            &[],
        ))
        .await
        .unwrap_err();
    assert_eq!(unknown.code(), Code::InvalidArgument);

    // TC-004 step 2: with `health_data` revoked, reading a profile that carries weight or height is refused
    // (`PERMISSION_DENIED`); accepting it again opens the read.
    use crate::proto::person::GetMeRequest;
    use crate::proto::person::person_service_server::PersonService;
    use crate::service::person_service::GrpcPersonService;
    use business::domain::user::User;
    w.db.execute_unprepared(&format!(
        "UPDATE person_info SET weight = 70.0, height = 175.0 WHERE person_id = {}",
        ada.person_id
    ))
    .await
    .unwrap();
    let people = GrpcPersonService::new(w.db.clone());
    let me = || {
        let mut request = tonic::Request::new(GetMeRequest {});
        request.extensions_mut().insert(User::new(
            Some("Ada".into()),
            "ada@example.test".into(),
            "hashed".into(),
            ada.person_id,
            ada.person_uuid.clone(),
        ));
        request
    };
    let refused = people.get_me(me()).await.unwrap_err();
    assert_eq!(refused.code(), Code::PermissionDenied, "{refused:?}");
    w.consent
        .accept_consent(with(accept("health_data", "1.0.0", true), Some(token), &[]))
        .await
        .unwrap();
    assert!(
        people.get_me(me()).await.is_ok(),
        "accepted again, the profile is readable"
    );
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn without_current_consent_only_the_recovery_paths_stay_open() {
    let mut w = world().await;
    let ada = register(&mut w, "ada@example.test").await;
    w.consent
        .revoke_consent(with(
            RevokeConsentRequest {
                document: "terms".into(),
            },
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap();
    let ended = w
        .consent
        .list_consents(with(ListConsentsRequest {}, Some(&ada.access_token), &[]))
        .await
        .unwrap_err();
    assert_eq!(
        (ended.code(), ended.message()),
        (Code::Unauthenticated, "token has been revoked"),
        "revoking Terms ends every session"
    );

    let ada = sign_in_again(&mut w, "ada@example.test").await;
    let token = ada.access_token.as_str();
    let ordinary = w
        .auth
        .deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(token), &[]))
        .await
        .unwrap_err();
    assert_eq!(
        (ordinary.code(), ordinary.message()),
        (Code::PermissionDenied, "terms consent is required"),
        "an ordinary call is refused"
    );

    let pending = w
        .consent
        .list_pending_consents(with(ListPendingConsentsRequest {}, Some(token), &[]))
        .await
        .unwrap()
        .into_inner();
    assert!(
        pending.pending.iter().any(|p| p.document == "terms"),
        "the consent services stay open"
    );
    w.consent
        .accept_consent(with(accept("terms", "1.0.0", true), Some(token), &[]))
        .await
        .unwrap();
    w.auth
        .deactivate_business_profile(with(DeactivateBusinessProfileRequest {}, Some(token), &[]))
        .await
        .expect("ordinary calls work again");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn account_deletion_is_requested_and_cancelled_even_without_current_consent() {
    let mut w = world().await;
    let ada = register(&mut w, "ada@example.test").await;
    w.consent
        .revoke_consent(with(
            RevokeConsentRequest {
                document: "terms".into(),
            },
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap();
    let ada = sign_in_again(&mut w, "ada@example.test").await;

    let nothing = w
        .account
        .cancel_account_deletion(with(
            CancelAccountDeletionRequest {},
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap_err();
    assert_eq!(
        (nothing.code(), error_key(&nothing)),
        (
            Code::InvalidArgument,
            ErrorKey::AccountDeletionNotPending.as_str()
        ),
        "no consent needed to ask, and nothing is pending"
    );

    let status = w
        .account
        .request_account_deletion(with(
            RequestAccountDeletionRequest { immediate: false },
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap()
        .into_inner();
    assert!(
        status.scheduled_at > status.requested_at,
        "after the grace period: {status:?}"
    );

    // The request ended every session; signing in again is allowed while the deletion is pending.
    let ada = sign_in_again(&mut w, "ada@example.test").await;
    assert!(
        ada.pending_account_deletion.is_some(),
        "the token tells the app a deletion is pending"
    );
    w.account
        .cancel_account_deletion(with(
            CancelAccountDeletionRequest {},
            Some(&ada.access_token),
            &[],
        ))
        .await
        .expect("cancel without consent");
    let twice = w
        .account
        .cancel_account_deletion(with(
            CancelAccountDeletionRequest {},
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap_err();
    assert_eq!(
        error_key(&twice),
        ErrorKey::AccountDeletionNotPending.as_str()
    );
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn data_exports_belong_to_their_owner_and_download_through_a_short_lived_link() {
    unsafe {
        std::env::set_var("AWS_WORKOUT_BUCKET", "c010-test-bucket");
        std::env::set_var("AWS_REGION", "us-east-1");
        std::env::set_var("AWS_ACCESS_KEY_ID", "test");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");
    }
    let mut w = world().await;
    let ada = register(&mut w, "ada@example.test").await;
    let bob = register(&mut w, "bob@example.test").await;

    let created = w
        .account
        .create_data_export(with(
            CreateDataExportRequest {},
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(created.status, "pending");
    let id = created.id.clone();
    let get =
        |token: &str, id: &str| with(GetDataExportRequest { id: id.into() }, Some(token), &[]);

    let own = w
        .account
        .list_data_exports(with(
            ListDataExportsRequest {},
            Some(&ada.access_token),
            &[],
        ))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        own.exports
            .iter()
            .map(|e| e.id.as_str())
            .collect::<Vec<_>>(),
        [id.as_str()]
    );
    let others = w
        .account
        .list_data_exports(with(
            ListDataExportsRequest {},
            Some(&bob.access_token),
            &[],
        ))
        .await
        .unwrap()
        .into_inner();
    assert!(
        others.exports.is_empty(),
        "another person's exports are not listed"
    );
    assert_eq!(
        w.account
            .get_data_export(get(&ada.access_token, &id))
            .await
            .unwrap()
            .into_inner()
            .id,
        id
    );
    let foreign = w
        .account
        .get_data_export(get(&bob.access_token, &id))
        .await
        .unwrap_err();
    assert_eq!(
        (foreign.code(), error_key(&foreign)),
        (Code::NotFound, ErrorKey::DataExportNotReady.as_str()),
        "to anyone else it does not exist"
    );
    let malformed = w
        .account
        .get_data_export(get(&ada.access_token, "not-a-uuid"))
        .await
        .unwrap_err();
    assert_eq!(malformed.code(), Code::InvalidArgument);

    let early = w
        .account
        .get_data_export_download_url(get(&ada.access_token, &id))
        .await
        .unwrap_err();
    assert_eq!(
        (early.code(), error_key(&early)),
        (Code::AlreadyExists, ErrorKey::DataExportNotReady.as_str()),
        "no link while the export is not ready"
    );

    w.db.execute_unprepared("UPDATE data_export SET status = 'ready', object_key = 'exports/ada.zip', expires_at = now() + interval '1 hour'").await.unwrap();
    let link = w
        .account
        .get_data_export_download_url(get(&ada.access_token, &id))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(link.expires_in_seconds, 900);
    assert!(
        link.url.contains("c010-test-bucket") && link.url.contains("exports/ada.zip"),
        "{}",
        link.url
    );
    let stolen = w
        .account
        .get_data_export_download_url(get(&bob.access_token, &id))
        .await
        .unwrap_err();
    assert_eq!(stolen.code(), Code::NotFound, "nobody else gets a link");
}
