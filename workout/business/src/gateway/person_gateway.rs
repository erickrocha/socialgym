use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{parse_uuid, parse_uuids};
use crate::domain::person::{Person, PersonEntityMapper};
use entity::person_entity as person;
use entity::person_entity::{ActiveModel, PersonEntity};
use entity::prelude::PersonEntity as PersonQuery;
use entity::prelude::UserEntity as UserQuery;
use entity::user_entity as user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr, DeleteResult,
    EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

pub struct PersonGateway {}

impl PersonGateway {
    pub async fn persist(db: &DbConn, entity: Person) -> Result<ActiveModel, DbErr> {
        let active_model = PersonEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    /// Deletes the `person` row as the final step of an account-purge cascade.
    /// Auto-cascades `exercise`, `workout_exercise`, and `person_media` per their
    /// `ON DELETE CASCADE` FKs — every other child table must be deleted first.
    pub async fn delete_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<DeleteResult, DbErr> {
        PersonQuery::delete_by_id(id).exec(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Option<PersonEntity> {
        PersonQuery::find()
            .filter(person::Column::Id.eq(id))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: &str) -> Result<Option<PersonEntity>, DbErr> {
        let uuid = parse_uuid(uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        PersonQuery::find()
            .filter(person::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn find_all(db: &DbConn) -> Vec<PersonEntity> {
        PersonQuery::find()
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_all_by_id_in(db: &DbConn, ids: Vec<i32>) -> Vec<PersonEntity> {
        PersonQuery::find()
            .filter(person::Column::Id.is_in(ids))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_all_by_uuid_in(db: &DbConn, ids: Vec<String>) -> Result<Vec<PersonEntity>, DbErr> {
        let ids = parse_uuids(&ids).map_err(|e| DbErr::Type(e.to_string()))?;
        PersonQuery::find()
            .filter(person::Column::Uuid.is_in(ids))
            .all(db)
            .await
    }

    pub async fn search_by_query(
        db: &DbConn,
        query: &str,
        exclude_person_id: i32,
        limit: u64,
    ) -> Vec<i32> {
        let search_pattern = format!("%{}%", query);

        PersonQuery::find()
            .filter(
                Condition::any()
                    .add(person::Column::FirstName.like(&search_pattern))
                    .add(person::Column::Surname.like(&search_pattern)),
            )
            .filter(person::Column::Id.ne(exclude_person_id))
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(|p| p.id)
            .collect()
    }

    pub async fn search_by_query_uuid(
        db: &DbConn,
        query: &str,
        exclude_person_uuid: String,
        limit: u64,
    ) -> Result<Vec<String>, DbErr> {
        let exclude_person_uuid = parse_uuid(&exclude_person_uuid)
            .map_err(|e| DbErr::Type(e.to_string()))?;
        let search_pattern = format!("%{}%", query);
        let people = PersonQuery::find()
            .filter(
                Condition::any()
                    .add(person::Column::FirstName.like(&search_pattern))
                    .add(person::Column::Surname.like(&search_pattern)),
            )
            .filter(person::Column::Uuid.ne(exclude_person_uuid))
            .limit(limit)
            .all(db)
            .await
            ?;
        Ok(people
            .into_iter()
            .map(|p| p.uuid.to_string())
            .collect())
    }

    pub async fn search_by_query_with_email(
        db: &DbConn,
        query: &str,
        exclude_person_id: i32,
        limit: u64,
    ) -> Vec<i32> {
        let search_pattern = format!("%{}%", query);

        let users: Vec<user::UserEntity> = UserQuery::find()
            .filter(
                Condition::any()
                    .add(user::Column::Email.like(&search_pattern))
                    .add(user::Column::Name.like(&search_pattern)),
            )
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new());

        users
            .into_iter()
            .filter(|u| u.person_id != exclude_person_id)
            .map(|u| u.person_id)
            .collect()
    }

    pub async fn search_by_query_with_email_uuid(
        db: &DbConn,
        query: &str,
        exclude_person_uuid: String,
        limit: u64,
    ) -> Result<Vec<String>, DbErr> {
        let exclude_person_uuid = parse_uuid(&exclude_person_uuid)
            .map_err(|e| DbErr::Type(e.to_string()))?;
        let search_pattern = format!("%{}%", query);

        let users: Vec<user::UserEntity> = UserQuery::find()
            .filter(
                Condition::any()
                    .add(user::Column::Email.like(&search_pattern))
                    .add(user::Column::Name.like(&search_pattern)),
            )
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new());

        Ok(users
            .into_iter()
            .filter(|u| u.person_uuid != exclude_person_uuid)
            .map(|u| u.person_uuid.to_string())
            .collect())
    }

    pub async fn search_friend_ids_by_username_or_email(
        db: &DbConn,
        query: &str,
        friend_ids: Vec<i32>,
        limit: u64,
    ) -> Vec<i32> {
        if friend_ids.is_empty() {
            return Vec::new();
        }

        let search_pattern = format!("%{}%", query);

        let users: Vec<user::UserEntity> = UserQuery::find()
            .filter(user::Column::PersonId.is_in(friend_ids))
            .filter(
                Condition::any()
                    .add(user::Column::Email.like(&search_pattern))
                    .add(user::Column::Name.like(&search_pattern)),
            )
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new());

        users.into_iter().map(|u| u.person_id).collect()
    }

    pub async fn search_friends_by_name(
        db: &DbConn,
        query: &str,
        friend_ids: Vec<i32>,
        limit: u64,
    ) -> Vec<i32> {
        if friend_ids.is_empty() {
            return Vec::new();
        }

        let search_pattern = format!("%{}%", query);

        PersonQuery::find()
            .join(JoinType::LeftJoin, entity::person_entity::Relation::User.def())
            .filter(person::Column::Id.is_in(friend_ids))
            .filter(
                Condition::any()
                    .add(person::Column::FirstName.like(&search_pattern))
                    .add(person::Column::Surname.like(&search_pattern))
                    .add(user::Column::Email.like(&search_pattern))
                    .add(user::Column::Name.like(&search_pattern)),
            )
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(|p| p.id)
            .collect()
    }
}
