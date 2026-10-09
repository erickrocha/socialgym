use crate::infrastructure::utils::{
    business_status, locale_of, localized_status, require_actor, require_person_id, validate_uuid,
};
use crate::proto::settings::settings_service_server::SettingsService;
use crate::proto::settings::{
    GetMySettingsRequest, OwnerUuidRequest, PushPreferenceResponse, Setting, SettingIdRequest,
    SettingOwnerIdRequest,
};
use business::commons::authorization::ensure_owns;
use business::commons::i18n::ErrorKey;
use business::domain::business_error::BusinessErrorKind;
use business::domain::enums::{Position, WeightUnit};
use business::domain::settings::Settings;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::setings_use_case::SettingsUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcSettingService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcSettingService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    fn domain_to_proto(settings: Settings) -> Setting {
        Setting {
            id: settings.id.unwrap_or(0),
            uuid: settings.uuid.unwrap_or_default(),
            owner_id: settings.person_id,
            owner_uuid: settings.person_uuid,
            language: settings.language,
            theme: settings.theme,
            notifications_enabled: settings.notifications_enabled,
            context_menu_position: settings.context_menu_position.to_string(),
            home_page: settings.home_page,
            weight_unit: settings
                .weight_unit
                .map(|u| u.to_string())
                .unwrap_or_default(),
            created_at: settings
                .created_at
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
            updated_at: settings
                .updated_at
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
        }
    }

    fn proto_to_domain(setting: Setting) -> Settings {
        Settings {
            id: if setting.id > 0 {
                Some(setting.id)
            } else {
                None
            },
            uuid: if setting.uuid.is_empty() {
                None
            } else {
                Some(setting.uuid)
            },
            person_id: setting.owner_id,
            person_uuid: setting.owner_uuid,
            language: setting.language,
            theme: setting.theme,
            notifications_enabled: setting.notifications_enabled,
            context_menu_position: Position::from_string(&setting.context_menu_position),
            home_page: setting.home_page,
            weight_unit: if setting.weight_unit.is_empty() {
                None
            } else {
                Some(WeightUnit::from_string(&setting.weight_unit))
            },
            created_at: None, // Will be set by database
            updated_at: None, // Will be set by database
        }
    }
}

#[tonic::async_trait]
impl SettingsService for GrpcSettingService {
    async fn get_push_preference_by_owner_uuid(
        &self,
        request: Request<OwnerUuidRequest>,
    ) -> Result<Response<PushPreferenceResponse>, Status> {
        let owner_uuid = request.into_inner().owner_uuid;
        validate_uuid(&owner_uuid, "owner_uuid")?;
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        match use_case.get_by_owner_uuid(owner_uuid).await {
            Ok(settings) => Ok(Response::new(PushPreferenceResponse {
                notifications_enabled: settings.notifications_enabled,
            })),
            Err(error) if error.kind == BusinessErrorKind::NotFound => {
                Err(Status::not_found("Settings not found"))
            }
            Err(_) => Err(Status::unavailable(
                "Settings service temporarily unavailable",
            )),
        }
    }

    async fn get_by_id(
        &self,
        request: Request<SettingIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let person_id = require_person_id(&request)?;
        let req = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let settings = use_case
            .get_by_id(req.id)
            .await
            .map_err(|_| Status::not_found("Settings not found"))?;
        ensure_owns(settings.person_id, person_id).map_err(business_status)?;
        Ok(Response::new(Self::domain_to_proto(settings)))
    }

    async fn persist_settings(
        &self,
        request: Request<Setting>,
    ) -> Result<Response<Setting>, Status> {
        let actor = require_actor(&request)?;
        let setting = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let domain_settings = Self::proto_to_domain(setting);
        let result = use_case.persist(domain_settings, &actor).await;

        match result {
            Ok(settings) => {
                let proto_setting = Self::domain_to_proto(settings);
                Ok(Response::new(proto_setting))
            }
            Err(_e) => Err(Status::internal("Error persisting settings")),
        }
    }

    async fn get_by_uuid(
        &self,
        request: Request<SettingIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let person_id = require_person_id(&request)?;
        let req = request.into_inner();
        validate_uuid(&req.uuid, "uuid")?;
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        let settings = use_case
            .get_by_uuid(req.uuid)
            .await
            .map_err(|_| Status::not_found("Settings not found"))?;
        ensure_owns(settings.person_id, person_id).map_err(business_status)?;
        Ok(Response::new(Self::domain_to_proto(settings)))
    }

    async fn get_by_owner_ids(
        &self,
        request: Request<SettingOwnerIdRequest>,
    ) -> Result<Response<Setting>, Status> {
        let actor = require_actor(&request)?;
        let req = request.into_inner();
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));

        // By uuid when no id is given, as the two REST routes do: the uuid must be the caller's own.
        if req.owner_id <= 0 && !req.owner_uuid.is_empty() {
            validate_uuid(&req.owner_uuid, "owner_uuid")?;
            if req.owner_uuid != actor.person_uuid {
                return Err(Status::permission_denied("Not the owner of this resource"));
            }
            let settings = use_case
                .get_by_owner_uuid(req.owner_uuid)
                .await
                .map_err(|_| Status::not_found("Settings not found"))?;
            return Ok(Response::new(Self::domain_to_proto(settings)));
        }

        ensure_owns(req.owner_id, actor.person_id).map_err(business_status)?;
        let settings = use_case
            .get_by_owner_id(req.owner_id)
            .await
            .map_err(|_| Status::not_found("Settings not found"))?;
        Ok(Response::new(Self::domain_to_proto(settings)))
    }

    async fn get_my_settings(
        &self,
        request: Request<GetMySettingsRequest>,
    ) -> Result<Response<Setting>, Status> {
        let locale = locale_of(&request);
        let person_id = require_person_id(&request)?;
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
        use_case
            .get_by_owner_id(person_id)
            .await
            .map(|settings| Response::new(Self::domain_to_proto(settings)))
            .map_err(|_| {
                localized_status(tonic::Code::NotFound, ErrorKey::SettingsNotFound, locale)
            })
    }

    async fn update_my_settings(
        &self,
        request: Request<Setting>,
    ) -> Result<Response<Setting>, Status> {
        let locale = locale_of(&request);
        let actor = require_actor(&request)?;
        let use_case = SettingsUseCase::new(SettingsGateway::new((*self.conn).clone()));
        // The owner is the person in the token: `persist` takes it from the actor, never the message.
        use_case
            .persist(Self::proto_to_domain(request.into_inner()), &actor)
            .await
            .map(|settings| Response::new(Self::domain_to_proto(settings)))
            .map_err(|_| {
                localized_status(
                    tonic::Code::InvalidArgument,
                    ErrorKey::SettingsUpdatedFailed,
                    locale,
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use business::domain::user::User;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectionTrait, Database};

    const OWNER: &str = "00000000-0000-0000-0000-000000000061";
    const OTHER: &str = "00000000-0000-0000-0000-000000000062";
    const LONELY: &str = "00000000-0000-0000-0000-000000000063";

    /// A request authenticated as `person_id`, as `GrpcAuthLayer` would deliver it.
    fn as_person<T>(message: T, person_id: i32, person_uuid: &str) -> Request<T> {
        let mut request = Request::new(message);
        request.extensions_mut().insert(User::new(
            Some("Caller".to_string()),
            "caller@example.test".to_string(),
            "hashed".to_string(),
            person_id,
            person_uuid.to_string(),
        ));
        request
    }

    fn by_uuid(uuid: &str) -> SettingIdRequest {
        SettingIdRequest {
            uuid: uuid.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn proto_and_domain_conversions_round_trip_and_default_missing_values() {
        let proto = Setting {
            id: 0,
            uuid: String::new(),
            owner_id: 7,
            owner_uuid: OWNER.to_string(),
            language: "pt".into(),
            theme: "dark".into(),
            notifications_enabled: true,
            context_menu_position: "Right".into(),
            home_page: "feed".into(),
            weight_unit: "Pounds".into(),
            ..Default::default()
        };
        let domain = GrpcSettingService::proto_to_domain(proto);
        assert_eq!(
            (domain.id, domain.uuid.clone()),
            (None, None),
            "unset ids stay unset"
        );
        assert_eq!(
            domain.weight_unit.as_ref().map(|unit| unit.to_string()),
            Some("Pounds".to_string())
        );
        let back = GrpcSettingService::domain_to_proto(domain);
        assert_eq!((back.id, back.uuid.as_str(), back.owner_id), (0, "", 7));
        assert_eq!(back.weight_unit, "Pounds");

        let unitless = GrpcSettingService::proto_to_domain(Setting {
            id: 3,
            uuid: "u".into(),
            ..Default::default()
        });
        assert_eq!(
            (unitless.id, unitless.uuid.as_deref()),
            (Some(3), Some("u"))
        );
        assert!(unitless.weight_unit.is_none());
        assert_eq!(
            GrpcSettingService::domain_to_proto(unitless).weight_unit,
            ""
        );
    }

    #[tokio::test]
    #[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
    async fn c006_settings_rpcs_enforce_ownership_and_expose_the_push_preference() {
        let url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
        let db = Database::connect(url.clone()).await.unwrap();
        Migrator::refresh(&db).await.unwrap();
        db.execute_unprepared(&format!(
            "INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at) VALUES
               (1, '{OWNER}', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
               (2, '{OTHER}', 'Other', 'Person', '1990-01-01', 'X', now(), now()),
               (3, '{LONELY}', 'Lonely', 'Person', '1990-01-01', 'X', now(), now());
             INSERT INTO settings (uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at) VALUES
               ('40000000-0000-0000-0000-000000000061', 1, '{OWNER}', 'en', 'light', true,  'Left', 'feed', now(), now()),
               ('40000000-0000-0000-0000-000000000062', 2, '{OTHER}', 'en', 'light', false, 'Left', 'feed', now(), now());"
        ))
        .await
        .unwrap();
        let service = GrpcSettingService::new(Arc::new(db));

        // Push preference: the internal RPC needs no actor, only a valid owner uuid.
        let preference = |uuid: &str| {
            let request = Request::new(OwnerUuidRequest {
                owner_uuid: uuid.to_string(),
            });
            service.get_push_preference_by_owner_uuid(request)
        };
        assert!(
            preference(OWNER)
                .await
                .unwrap()
                .into_inner()
                .notifications_enabled
        );
        assert!(
            !preference(OTHER)
                .await
                .unwrap()
                .into_inner()
                .notifications_enabled
        );
        assert_eq!(
            preference(LONELY).await.unwrap_err().code(),
            tonic::Code::NotFound
        );
        assert_eq!(
            preference("not-a-uuid").await.unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
        let closed = Database::connect(url).await.unwrap();
        closed.close_by_ref().await.unwrap();
        let offline = GrpcSettingService::new(Arc::new(closed));
        let request = Request::new(OwnerUuidRequest {
            owner_uuid: OWNER.to_string(),
        });
        assert_eq!(
            offline
                .get_push_preference_by_owner_uuid(request)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::Unavailable,
            "a database outage is retryable, not a permanent failure"
        );

        // Reads require an authenticated caller and are limited to the caller's own settings.
        let anonymous = Request::new(SettingIdRequest {
            id: 1,
            ..Default::default()
        });
        assert_eq!(
            service.get_by_id(anonymous).await.unwrap_err().code(),
            tonic::Code::Unauthenticated
        );

        let own = service
            .get_by_id(as_person(
                SettingIdRequest {
                    id: 1,
                    ..Default::default()
                },
                1,
                OWNER,
            ))
            .await;
        let own = own.unwrap().into_inner();
        assert_eq!(
            (
                own.owner_id,
                own.owner_uuid.as_str(),
                own.notifications_enabled
            ),
            (1, OWNER, true)
        );
        let foreign = service
            .get_by_id(as_person(
                SettingIdRequest {
                    id: 2,
                    ..Default::default()
                },
                1,
                OWNER,
            ))
            .await;
        assert_eq!(foreign.unwrap_err().code(), tonic::Code::PermissionDenied);
        let missing = service
            .get_by_id(as_person(
                SettingIdRequest {
                    id: 9999,
                    ..Default::default()
                },
                1,
                OWNER,
            ))
            .await;
        assert_eq!(missing.unwrap_err().code(), tonic::Code::NotFound);

        let by_own_uuid = service
            .get_by_uuid(as_person(
                by_uuid("40000000-0000-0000-0000-000000000061"),
                1,
                OWNER,
            ))
            .await;
        assert_eq!(by_own_uuid.unwrap().into_inner().owner_id, 1);
        let by_foreign = service
            .get_by_uuid(as_person(
                by_uuid("40000000-0000-0000-0000-000000000062"),
                1,
                OWNER,
            ))
            .await;
        assert_eq!(
            by_foreign.unwrap_err().code(),
            tonic::Code::PermissionDenied
        );
        let bad_uuid = service
            .get_by_uuid(as_person(by_uuid("nope"), 1, OWNER))
            .await;
        assert_eq!(bad_uuid.unwrap_err().code(), tonic::Code::InvalidArgument);
        let unknown = service
            .get_by_uuid(as_person(
                by_uuid("40000000-0000-0000-0000-0000000000ff"),
                1,
                OWNER,
            ))
            .await;
        assert_eq!(unknown.unwrap_err().code(), tonic::Code::NotFound);

        let owner_ids = |caller: i32, owner: i32| {
            service.get_by_owner_ids(as_person(
                SettingOwnerIdRequest {
                    owner_id: owner,
                    ..Default::default()
                },
                caller,
                OWNER,
            ))
        };
        assert_eq!(
            owner_ids(1, 1).await.unwrap().into_inner().owner_uuid,
            OWNER
        );
        assert_eq!(
            owner_ids(1, 2).await.unwrap_err().code(),
            tonic::Code::PermissionDenied
        );
        assert_eq!(
            owner_ids(3, 3).await.unwrap_err().code(),
            tonic::Code::NotFound
        );

        // Writes: the owner always comes from the caller, never the payload.
        let update = Setting {
            id: 1,
            owner_id: 1,
            owner_uuid: OWNER.into(),
            language: "pt".into(),
            theme: "dark".into(),
            notifications_enabled: false,
            context_menu_position: "Right".into(),
            home_page: "feed".into(),
            ..Default::default()
        };
        let saved = service
            .persist_settings(as_person(update.clone(), 1, OWNER))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(
            (saved.language.as_str(), saved.notifications_enabled),
            ("pt", false)
        );
        assert!(
            !preference(OWNER)
                .await
                .unwrap()
                .into_inner()
                .notifications_enabled,
            "the RPC sees the new preference"
        );

        let hijack = service
            .persist_settings(as_person(
                Setting {
                    id: 2,
                    ..update.clone()
                },
                1,
                OWNER,
            ))
            .await;
        assert_eq!(
            hijack.unwrap_err().code(),
            tonic::Code::Internal,
            "another person's row is not writable"
        );
        let duplicate = service
            .persist_settings(as_person(
                Setting {
                    id: 0,
                    ..update.clone()
                },
                1,
                OWNER,
            ))
            .await;
        assert_eq!(
            duplicate.unwrap_err().code(),
            tonic::Code::Internal,
            "one settings row per person"
        );
        let created = service
            .persist_settings(as_person(
                Setting {
                    id: 0,
                    owner_id: 99,
                    ..update
                },
                3,
                LONELY,
            ))
            .await;
        assert_eq!(
            created.unwrap().into_inner().owner_id,
            3,
            "owner is taken from the caller"
        );
        assert_eq!(
            service
                .persist_settings(Request::new(Setting::default()))
                .await
                .unwrap_err()
                .code(),
            tonic::Code::Unauthenticated
        );
    }
}
