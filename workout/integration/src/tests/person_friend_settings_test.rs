//! C-010 task 6: the friend profile, the caller's own settings and the person-address and settings reads by
//! uuid over gRPC (TC-008). Handler level, against a disposable PostGIS database (`TEST_DATABASE_URL`).
use super::server_support::error_key;
use crate::proto::friend::friend_service_server::FriendService;
use crate::proto::friend::FriendProfileRequest;
use crate::proto::settings::settings_service_server::SettingsService;
use crate::proto::settings::{GetMySettingsRequest, Setting};
use crate::service::friend_service::GrpcFriendService;
use crate::service::settings_service::GrpcSettingService;
use business::commons::i18n::ErrorKey;
use business::domain::user::User;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use std::sync::Arc;
use tonic::{Code, Request};

const UUIDS: [&str; 4] = [
    "00000000-0000-0000-0000-0000000000e1",
    "00000000-0000-0000-0000-0000000000e2",
    "00000000-0000-0000-0000-0000000000e3",
    "00000000-0000-0000-0000-0000000000e4",
];

fn as_person<T>(message: T, id: i32) -> Request<T> {
    let mut request = Request::new(message);
    request.extensions_mut().insert(User::new(Some(format!("Person {id}")), format!("p{id}@example.test"), "hashed".into(), id, UUIDS[(id - 1) as usize].into()));
    request
}

/// Alice (1) and Bob (2) are friends, Alice asked Dave (4) who has not answered, Carol (3) is a stranger.
/// Bob has weight and height on file; Alice and Carol have settings.
async fn world() -> Arc<DatabaseConnection> {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(&format!(
        r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
             (1, '{a}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now()),
             (2, '{b}', 'Bob', 'Friend', '1990-01-01', 'X', now(), now()),
             (3, '{c}', 'Carol', 'Stranger', '1990-01-01', 'X', now(), now()),
             (4, '{d}', 'Dave', 'Pending', '1990-01-01', 'X', now(), now());
           INSERT INTO person_info (id, uuid, person_id, weight, height, created_at, updated_at) VALUES
             (1, '10000000-0000-0000-0000-0000000000e2', 2, 80.0, 180.0, now(), now());
           INSERT INTO friends (uuid, person_id, person_uuid, friend_id, friend_uuid, status, created_at, updated_at) VALUES
             ('60000000-0000-0000-0000-0000000000e1', 1, '{a}', 2, '{b}', 'Accepted', now(), now()),
             ('60000000-0000-0000-0000-0000000000e2', 2, '{b}', 1, '{a}', 'Accepted', now(), now()),
             ('60000000-0000-0000-0000-0000000000e3', 1, '{a}', 4, '{d}', 'Pending', now(), now());
           INSERT INTO settings (uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at) VALUES
             ('40000000-0000-0000-0000-0000000000e1', 1, '{a}', 'en', 'light', true, 'Left', 'feed', now(), now()),
             ('40000000-0000-0000-0000-0000000000e3', 3, '{c}', 'en', 'dark', false, 'Left', 'feed', now(), now())"#,
        a = UUIDS[0], b = UUIDS[1], c = UUIDS[2], d = UUIDS[3]
    ))
    .await
    .unwrap();
    Arc::new(db)
}

fn friend_profile(id: i32) -> FriendProfileRequest {
    FriendProfileRequest { friend_id: id }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn a_friend_profile_is_served_only_for_an_accepted_friendship() {
    let service = GrpcFriendService::new(world().await);

    let bob = service.get_friend_profile(as_person(friend_profile(2), 1)).await.expect("a friend").into_inner();
    assert_eq!(bob.person.as_ref().map(|p| p.uuid.as_str()), Some(UUIDS[1]));

    // Everyone who is not an accepted friend gets the same refusal, so the answer does not say who exists.
    for stranger in [3, 4, 999] {
        let status = service.get_friend_profile(as_person(friend_profile(stranger), 1)).await.unwrap_err();
        assert_eq!((status.code(), error_key(&status)), (Code::PermissionDenied, ErrorKey::FriendNotFound.as_str()), "friend id {stranger}");
    }
    let myself = service.get_friend_profile(as_person(friend_profile(1), 1)).await.unwrap_err();
    assert_eq!(myself.code(), Code::InvalidArgument, "the caller's own id does not go through the friend path");
    let anonymous = service.get_friend_profile(Request::new(friend_profile(2))).await.unwrap_err();
    assert_eq!(anonymous.code(), Code::Unauthenticated);
}

/// W18 (owner decision 2026-10-07): the friend profile hides the friend's weight and height, as REST
/// `GET /people/me/friend/{id}` does; the rest of `person_info` is still shown.
#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn w18_the_friend_profile_hides_the_friends_health_data() {
    let service = GrpcFriendService::new(world().await);
    let bob = service.get_friend_profile(as_person(friend_profile(2), 1)).await.unwrap().into_inner().person.unwrap();
    let info = bob.person_info.expect("person info");
    assert_eq!((info.weight, info.height), (0.0, 0.0));
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_callers_own_settings_are_read_and_updated_from_the_token() {
    let service = GrpcSettingService::new(world().await);

    let own = service.get_my_settings(as_person(GetMySettingsRequest {}, 1)).await.unwrap().into_inner();
    assert_eq!((own.owner_uuid.as_str(), own.theme.as_str()), (UUIDS[0], "light"));
    let none = service.get_my_settings(as_person(GetMySettingsRequest {}, 2)).await.unwrap_err();
    assert_eq!((none.code(), error_key(&none)), (Code::NotFound, ErrorKey::SettingsNotFound.as_str()), "Bob has no settings row");

    // The message names Carol as the owner; the update still lands on the caller.
    let update = Setting {
        id: own.id,
        owner_id: 3,
        owner_uuid: UUIDS[2].into(),
        language: "pt".into(),
        theme: "dark".into(),
        notifications_enabled: false,
        context_menu_position: "Right".into(),
        home_page: "feed".into(),
        ..Default::default()
    };
    let saved = service.update_my_settings(as_person(update, 1)).await.expect("update").into_inner();
    assert_eq!((saved.owner_id, saved.language.as_str(), saved.theme.as_str()), (1, "pt", "dark"));
    let again = service.get_my_settings(as_person(GetMySettingsRequest {}, 1)).await.unwrap().into_inner();
    assert_eq!(again.theme, "dark");
    let carol = service.get_my_settings(as_person(GetMySettingsRequest {}, 3)).await.unwrap().into_inner();
    assert_eq!(carol.language, "en", "Carol's settings were not touched");

    let anonymous = service.get_my_settings(Request::new(GetMySettingsRequest {})).await.unwrap_err();
    assert_eq!(anonymous.code(), Code::Unauthenticated);
}
