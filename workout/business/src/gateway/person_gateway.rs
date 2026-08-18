use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::string_to_uuid;
use crate::domain::person::{Person, PersonEntityMapper};
use entity::person;
use entity::person::{ActiveModel, Model};
use entity::prelude::Person as PersonQuery;
use entity::prelude::User as UserQuery;
use entity::user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, DbErr, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait,
};
use uuid::Uuid;

pub struct PersonGateway {}

impl PersonGateway {
    pub async fn persist(db: &DbConn, entity: Person) -> Result<ActiveModel, DbErr> {
        let active_model = PersonEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Option<Model> {
        PersonQuery::find()
            .filter(person::Column::Id.eq(id))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: &str) -> Option<Model> {
        PersonQuery::find()
            .filter(person::Column::Uuid.eq(string_to_uuid(uuid)))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_all(db: &DbConn) -> Vec<Model> {
        PersonQuery::find()
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_all_by_id_in(db: &DbConn, ids: Vec<i32>) -> Vec<Model> {
        PersonQuery::find()
            .filter(person::Column::Id.is_in(ids))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_all_by_uuid_in(db: &DbConn, ids: Vec<Uuid>) -> Vec<Model> {
        PersonQuery::find()
            .filter(person::Column::Uuid.is_in(ids))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
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
        exclude_person_uuid: Uuid,
        limit: u64,
    ) -> Vec<Uuid> {
        let search_pattern = format!("%{}%", query);
        PersonQuery::find()
            .filter(
                Condition::any()
                    .add(person::Column::FirstName.like(&search_pattern))
                    .add(person::Column::Surname.like(&search_pattern)),
            )
            .filter(person::Column::Uuid.ne(exclude_person_uuid))
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(|p| p.uuid)
            .collect()
    }

    pub async fn search_by_query_with_email(
        db: &DbConn,
        query: &str,
        exclude_person_id: i32,
        limit: u64,
    ) -> Vec<i32> {
        let search_pattern = format!("%{}%", query);

        let users: Vec<user::Model> = UserQuery::find()
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
        exclude_person_uuid: Uuid,
        limit: u64,
    ) -> Vec<Uuid> {
        let search_pattern = format!("%{}%", query);

        let users: Vec<user::Model> = UserQuery::find()
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
            .filter(|u| u.person_uuid != exclude_person_uuid)
            .map(|u| u.person_uuid)
            .collect()
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

        let users: Vec<user::Model> = UserQuery::find()
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
            .join(JoinType::LeftJoin, entity::person::Relation::User.def())
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
