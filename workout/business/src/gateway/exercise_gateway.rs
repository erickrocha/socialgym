use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{parse_uuid, parse_uuids};
use crate::domain::enums::Visibility;
use crate::domain::exercise::{Exercise, ExerciseEntityMapper};
use entity::exercise_entity as exercise;
use entity::exercise_entity::{Column, Entity};
use entity::prelude::ExerciseEntity as ExerciseQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, DbErr, DeleteResult, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder,
};

pub struct ExerciseGateway {}

impl ExerciseGateway {
    pub async fn find_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_by_uuid(
        db: &DbConn,
        uuid: String,
    ) -> Result<Option<exercise::ExerciseEntity>, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        ExerciseQuery::find()
            .filter(Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<i32>,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::Id.is_in(ids))
            .all(db)
            .await
    }

    pub async fn persist(db: &DbConn, entity: Exercise) -> Result<exercise::ActiveModel, DbErr> {
        let active_model = ExerciseEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, entity: Exercise) -> Result<exercise::ExerciseEntity, DbErr> {
        let active_model = ExerciseEntityMapper::build_active_model(entity);
        active_model.update(db).await
    }

    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        ExerciseQuery::delete_by_id(id).exec(db).await
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<DeleteResult, DbErr> {
        let uuid = parse_uuid(&uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        Entity::delete_many()
            .filter(Column::Uuid.eq(uuid))
            .exec(db)
            .await
    }

    pub async fn find_by_owner_id(
        db: &DbConn,
        owner_id: i32,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::OwnerId.eq(owner_id))
            .all(db)
            .await
    }

    pub async fn find_by_owner_ids(
        db: &DbConn,
        owner_ids: Vec<i32>,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::OwnerId.is_in(owner_ids))
            .all(db)
            .await
    }

    pub async fn find_all(db: &DbConn) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find().all(db).await
    }

    pub async fn find_by_visibility(
        db: &DbConn,
        visibility: String,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::Visibility.eq(visibility))
            .all(db)
            .await
    }

    pub async fn find_by_owner_id_and_visibility(
        db: &DbConn,
        owner_id: i32,
        visibility: String,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::OwnerId.eq(owner_id))
            .filter(Column::Visibility.eq(visibility))
            .all(db)
            .await
    }

    pub async fn find_by_owner_ids_and_visibility(
        db: &DbConn,
        owner_ids: Vec<i32>,
        visibility: String,
    ) -> Result<Vec<exercise::ExerciseEntity>, DbErr> {
        ExerciseQuery::find()
            .filter(Column::OwnerId.is_in(owner_ids))
            .filter(Column::Visibility.eq(visibility))
            .all(db)
            .await
    }

    pub async fn find_by_filters_paginated(
        db: &DbConn,
        owner_name: Option<String>,
        category: Option<String>,
        visibility: String,
        offset: u64,
        limit: u64,
        sort_by: Option<String>,
    ) -> Result<(Vec<exercise::ExerciseEntity>, u64), DbErr> {
        let mut query = ExerciseQuery::find().filter(Column::Visibility.eq(visibility));

        if let Some(name) = owner_name {
            query = query.filter(Column::OwnerName.like(format!("%{}%", name)));
        }

        if let Some(cat) = category {
            query = query.filter(Column::Category.eq(cat));
        }

        let sort = sort_by.unwrap_or_else(|| "created_at_desc".to_string());
        query = match sort.as_str() {
            "created_at_asc" => query.order_by(Column::CreatedAt, Order::Asc),
            "owner_name_asc" => query.order_by(Column::OwnerName, Order::Asc),
            "name_asc" => query.order_by(Column::Name, Order::Asc),
            _ => query.order_by(Column::CreatedAt, Order::Desc),
        };

        let total_count = query.clone().count(db).await?;

        let models = query.paginate(db, limit).fetch_page(offset).await?;

        Ok((models, total_count))
    }

    pub async fn count_by_filters(
        db: &DbConn,
        owner_name: Option<String>,
        category: Option<String>,
        visibility: String,
    ) -> Result<u64, DbErr> {
        let mut query = ExerciseQuery::find().filter(Column::Visibility.eq(visibility));
        if let Some(name) = owner_name {
            query = query.filter(Column::OwnerName.like(format!("%{}%", name)));
        }
        if let Some(cat) = category {
            query = query.filter(Column::Category.eq(cat));
        }
        query.count(db).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_by_complex_filters_paginated(
        db: &DbConn,
        current_user_owner_id: i32,
        friend_ids: Vec<i32>,
        public_owner_ids: Vec<i32>,
        category: Option<String>,
        visibility: Option<String>,
        offset: u64,
        limit: u64,
        sort_by: Option<String>,
    ) -> Result<(Vec<exercise::ExerciseEntity>, u64), DbErr> {
        let mut query = ExerciseQuery::find().filter(
            Condition::any()
                .add(Column::OwnerId.eq(current_user_owner_id))
                .add(
                    Condition::all()
                        .add(Column::OwnerId.is_in(friend_ids.clone()))
                        .add(
                            Condition::any()
                                .add(Column::Visibility.eq(Visibility::Friends.to_string()))
                                .add(Column::Visibility.eq(Visibility::Public.to_string())),
                        ),
                )
                .add(
                    Condition::all()
                        .add(Column::OwnerId.is_in(public_owner_ids))
                        .add(Column::Visibility.eq(Visibility::Public.to_string())),
                ),
        );

        if let Some(cat) = category {
            query = query.filter(Column::Category.eq(cat));
        }

        if let Some(vis) = visibility {
            query = query.filter(Column::Visibility.eq(Visibility::from_string(&vis).to_string()));
        }

        let sort = sort_by.unwrap_or_else(|| "created_at_desc".to_string());
        query = match sort.as_str() {
            "created_at_asc" => query.order_by(Column::CreatedAt, Order::Asc),
            "owner_name_asc" => query.order_by(Column::OwnerName, Order::Asc),
            "name_asc" => query.order_by(Column::Name, Order::Asc),
            _ => query.order_by(Column::CreatedAt, Order::Desc),
        };

        let total_count = query.clone().count(db).await?;

        let models = query.paginate(db, limit).fetch_page(offset).await?;

        Ok((models, total_count))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_by_complex_filters_paginated_uuid(
        db: &DbConn,
        current_user_owner_uuid: String,
        friend_uuids: Vec<String>,
        public_owner_uuids: Vec<String>,
        category: Option<String>,
        visibility: Option<String>,
        offset: u64,
        limit: u64,
        sort_by: Option<String>,
    ) -> Result<(Vec<exercise::ExerciseEntity>, u64), DbErr> {
        let current_user_owner_uuid =
            parse_uuid(&current_user_owner_uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        let friend_uuids = parse_uuids(&friend_uuids).map_err(|e| DbErr::Type(e.to_string()))?;
        let public_owner_uuids =
            parse_uuids(&public_owner_uuids).map_err(|e| DbErr::Type(e.to_string()))?;
        let mut query = ExerciseQuery::find().filter(
            Condition::any()
                .add(Column::OwnerUuid.eq(current_user_owner_uuid))
                .add(
                    Condition::all()
                        .add(Column::OwnerUuid.is_in(friend_uuids.clone()))
                        .add(
                            Condition::any()
                                .add(Column::Visibility.eq(Visibility::Friends.to_string()))
                                .add(Column::Visibility.eq(Visibility::Public.to_string())),
                        ),
                )
                .add(
                    Condition::all()
                        .add(Column::OwnerUuid.is_in(public_owner_uuids.clone()))
                        .add(Column::Visibility.eq(Visibility::Public.to_string())),
                ),
        );

        if let Some(cat) = category {
            query = query.filter(Column::Category.eq(cat));
        }

        if let Some(vis) = visibility {
            query = query.filter(Column::Visibility.eq(Visibility::from_string(&vis).to_string()));
        }

        let sort = sort_by.unwrap_or_else(|| "created_at_desc".to_string());
        query = match sort.as_str() {
            "created_at_asc" => query.order_by(Column::CreatedAt, Order::Asc),
            "owner_name_asc" => query.order_by(Column::OwnerName, Order::Asc),
            "name_asc" => query.order_by(Column::Name, Order::Asc),
            _ => query.order_by(Column::CreatedAt, Order::Desc),
        };

        let total_count = query.clone().count(db).await?;

        let models = query.paginate(db, limit).fetch_page(offset).await?;

        Ok((models, total_count))
    }
}
