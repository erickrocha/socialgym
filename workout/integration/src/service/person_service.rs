use crate::infrastructure::mapper::{Mapper, PersonAddressMapper, PersonInfoMapper, PersonMapper};
use crate::proto::person::person_id_request::Identifier;
use crate::proto::person::person_service_server::PersonService;
use crate::proto::person::{
    DeletePersonImageResponse, GetMeRequest, PeopleResponse, Person, PersonIdRequest,
    PersonImageRequest, PersonImageUploadRequest, PersonImageUploadResponse, PersonParams,
    PersonResponse, RemovePersonAddressRequest, RemovePersonAddressResponse,
    SearchMentionableFriendsRequest,
};
use business::domain::person::Person as DomainPerson;
use business::gateway::person_address_gateway::PersonAddressGateway;
use business::gateway::person_info_gateway::PersonInfoGateway;
use business::use_cases::common_use_case::choose_image_type;
use business::use_cases::person_address_use_case::PersonAddressUseCase;
use business::use_cases::person_info_use_case::PersonInfoUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::infrastructure::utils::{require_person_id, validate_uuid};
use crate::proto::person::person_params::ParamIdentifier;

pub struct GrpcPersonService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcPersonService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

/// Pulls the authenticated caller's `person_id` out of the request extensions
/// inserted by `GrpcAuthLayer`. Every RPC on this service runs behind that
/// layer, which either populates this extension on success or short-circuits
/// the request with `UNAUTHENTICATED` before the handler ever runs.
#[tonic::async_trait]
impl PersonService for GrpcPersonService {
    async fn get_person(&self,request: Request<PersonIdRequest>) -> Result<Response<PersonResponse>, Status> {
        let req = request.into_inner();

        match req.identifier {
            Some(Identifier::Id(id)) => {
                if id <= 0 {
                    return Err(Status::invalid_argument("id must be a positive integer"));
                }
                let person = PersonUseCase::get(&self.conn, id)
                    .await
                    .map_err(|e| Status::internal(e.message))?;

                let grpc_person = PersonMapper::response(person);

                Ok(Response::new(PersonResponse {
                    person: Some(grpc_person),
                }))
            }
            Some(Identifier::Uuid(uuid)) => {
                if uuid.is_empty() {
                    return Err(Status::invalid_argument("uuid must be informed"));
                }
                validate_uuid(&uuid, "uuid")?;
                let person = PersonUseCase::find_by_uuid(&self.conn, uuid)
                    .await
                    .map_err(|e| Status::internal(e.message))?;
                let grpc_person = PersonMapper::response(person);

                Ok(Response::new(PersonResponse {
                    person: Some(grpc_person),
                }))
            }
            None =>  Err(Status::invalid_argument("either id or uuid must be informed")),
        }
    }

    async fn get_me(&self, request: Request<GetMeRequest>) -> Result<Response<PersonResponse>, Status> {
        let person_id = require_person_id(&request)?;

        let person = PersonUseCase::get(&self.conn, person_id)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(PersonResponse {
            person: Some(PersonMapper::response(person)),
        }))
    }

    async fn search_mentionable_friends(&self,request: Request<SearchMentionableFriendsRequest>) -> Result<Response<PeopleResponse>, Status> {
        let payload = request.into_inner();

        if payload.person_id <= 0 {
            return Err(Status::invalid_argument("person_id must be informed"));
        }

        let query = payload.query.trim().trim_start_matches('@').to_string();
        if query.is_empty() {
            return Ok(Response::new(PeopleResponse { people: vec![] }));
        }

        let requested_limit = if payload.limit > 0 { payload.limit } else { 10 };
        let limit = requested_limit.min(20);

        let people =
            PersonUseCase::search_mentionable_friends(&self.conn, payload.person_id, &query, limit)
                .await;

        Ok(Response::new(PeopleResponse {
            people: PersonMapper::response_vec(people),
        }))
    }

    async fn update_person( &self,request: Request<Person>) -> Result<Response<PersonResponse>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        let existing = PersonUseCase::get(&self.conn, person_id)
            .await
            .map_err(|e| Status::internal(e.message))?;

        let firstname = if payload.firstname.is_empty() { existing.firstname } else { payload.firstname };
        let surname = if payload.surname.is_empty() { existing.surname } else { payload.surname };
        let date_of_birth = if payload.date_of_birth.is_empty() {
            existing.date_of_birth
        } else {
            chrono::NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")
                .unwrap_or(existing.date_of_birth)
        };
        let gender = if payload.gender.is_empty() { existing.gender } else { payload.gender };
        let avatar = if payload.avatar.is_empty() { existing.avatar.unwrap_or_default() } else { payload.avatar };
        let cover = if payload.cover.is_empty() { existing.cover.unwrap_or_default() } else { payload.cover };
        let person_info = match payload.person_info {
            Some(info) => Some(PersonInfoMapper::domain(info)),
            None => existing.person_info,
        };

        let person = DomainPerson::update(
            Some(person_id),
            firstname,
            surname,
            date_of_birth,
            gender,
            Some(avatar),
            Some(cover),
            None,
            person_info,
            Vec::new(),
            Vec::new(),
        );

        let person_response = PersonUseCase::update(&self.conn, person).await;

        match person_response {
            Ok(person) => Ok(Response::new(PersonResponse {
                person: Some(PersonMapper::response(person)),
            })),
            Err(e) => Err(Status::internal(e.message)),
        }
    }

    async fn update_person_info(&self, request: Request<crate::proto::person_info::PersonInfo>) -> Result<Response<crate::proto::person_info::PersonInfo>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        let existing = PersonInfoGateway::find_by_person_id(&self.conn, person_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("PersonInfo not found"))?;

        let domain = PersonInfoMapper::domain(payload);

        let updated = PersonInfoUseCase::update(&self.conn, existing.id, domain)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(PersonInfoMapper::response(updated)))
    }

    async fn add_person_address(&self, request: Request<crate::proto::person_address::PersonAddress>) -> Result<Response<crate::proto::person_address::PersonAddress>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        let mut address = PersonAddressMapper::domain(payload);
        address.id = None;
        address.person_id = person_id;

        let added = PersonAddressUseCase::add_person_address(&self.conn, address)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(PersonAddressMapper::response(added)))
    }

    async fn update_person_address(&self, request: Request<crate::proto::person_address::PersonAddress>) -> Result<Response<crate::proto::person_address::PersonAddress>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        if payload.id <= 0 {
            return Err(Status::invalid_argument("id must be a positive integer"));
        }

        let existing = PersonAddressGateway::find_by_id(&self.conn, payload.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Person address not found"))?;

        if existing.person_id != person_id {
            return Err(Status::permission_denied("address does not belong to the authenticated person"));
        }

        let mut address = PersonAddressMapper::domain(payload);
        address.id = Some(existing.id);
        address.person_id = person_id;

        let updated = PersonAddressUseCase::update_person_address(&self.conn, address, person_id)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(PersonAddressMapper::response(updated)))
    }

    async fn remove_person_address(&self, request: Request<RemovePersonAddressRequest>) -> Result<Response<RemovePersonAddressResponse>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        if payload.id <= 0 {
            return Err(Status::invalid_argument("id must be a positive integer"));
        }

        let existing = PersonAddressGateway::find_by_id(&self.conn, payload.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Person address not found"))?;

        if existing.person_id != person_id {
            return Err(Status::permission_denied("address does not belong to the authenticated person"));
        }

        PersonAddressUseCase::delete_person_address(&self.conn, existing.id, person_id)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(RemovePersonAddressResponse { success: true }))
    }

    async fn get_person_image_upload_url(&self, request: Request<PersonImageUploadRequest>) -> Result<Response<PersonImageUploadResponse>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        let format = if payload.format.is_empty() { "jpg".to_string() } else { payload.format };
        let image_type = choose_image_type(payload.image_type.as_str());

        let image_storage = PersonUseCase::upload_person_image(&self.conn, person_id, image_type, format)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(PersonImageUploadResponse {
            url: image_storage.url,
            object_key: image_storage.object_key,
            person_id,
        }))
    }

    async fn delete_person_image(&self, request: Request<PersonImageRequest>) -> Result<Response<DeletePersonImageResponse>, Status> {
        let person_id = require_person_id(&request)?;
        let payload = request.into_inner();

        let image_type = choose_image_type(payload.image_type.as_str());

        PersonUseCase::delete_person_image(&self.conn, person_id, image_type)
            .await
            .map_err(|e| Status::internal(e.message))?;

        Ok(Response::new(DeletePersonImageResponse { success: true }))
    }

    async fn search_persons(&self,request: Request<PersonParams>) -> Result<Response<PeopleResponse>, Status> {
        let req = request.into_inner();

        let query = req.query;
        let limit = req.limit;

        match req.param_identifier {
            Some(ParamIdentifier::Id(id)) => {
                if id <= 0 {
                    return Err(Status::invalid_argument("id must be a positive integer"));
                }
                // Cap the limit at 100
                let limit = if limit > 100 { 100 } else if limit < 1 { 50 } else { limit };
                let persons = PersonUseCase::search_persons(&self.conn, &query, id, limit).await;
                Ok(Response::new(PeopleResponse {
                    people: PersonMapper::response_vec(persons),
                }))
            }
            Some(ParamIdentifier::Uuid(uuid)) => {
                if uuid.is_empty() {
                    return Err(Status::invalid_argument("uuid must be informed"));
                }
                validate_uuid(&uuid, "uuid")?;
                // Cap the limit at 100
                let limit = if limit > 100 { 100 } else if limit < 1 { 50 } else { limit };
                let persons = PersonUseCase::search_persons_by_uuid(&self.conn, &query, uuid, limit).await;
                Ok(Response::new(PeopleResponse {
                    people: PersonMapper::response_vec(persons),
                }))
            }
            None => Err(Status::invalid_argument("either id or uuid must be informed")),
        }
    }
}
