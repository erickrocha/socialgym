use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::parse_uuid;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use entity::business_profile_entity::{ActiveModel, BusinessProfileEntity};
use entity::prelude::BusinessProfileEntity as BusinessProfileQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr, DeleteResult,
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

/// Escapes `\`, `%` and `_` so user text is matched literally in a `LIKE` pattern
/// (Postgres uses backslash as the default escape character).
pub fn escape_like(text: &str) -> String {
    text.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

pub struct BusinessProfileGateway {}

impl BusinessProfileGateway {
    pub async fn persist(db: &DbConn, entity: BusinessProfile) -> Result<ActiveModel, DbErr> {
        let active_model = BusinessProfileEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Option<BusinessProfileEntity> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::Id.eq(id))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: &str) -> Result<Option<BusinessProfileEntity>, DbErr> {
        let uuid = parse_uuid(uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::Uuid.eq(uuid))
            .one(db)
            .await
    }

    pub async fn find_by_owner_id(db: &DbConn, owner_id: i32) -> Vec<BusinessProfileEntity> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::OwnerId.eq(owner_id))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_by_owner_uuid(db: &DbConn, owner_uuid: &str) -> Result<Vec<BusinessProfileEntity>, DbErr> {
        let owner_uuid = parse_uuid(owner_uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::OwnerUuid.eq(owner_uuid))
            .all(db)
            .await
    }

    pub async fn count_by_owner_id(db: &DbConn, owner_id: i32) -> u64 {
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::OwnerId.eq(owner_id))
            .count(db)
            .await
            .unwrap_or(0)
    }

    pub async fn find_all_by_ids(db: &DbConn, ids: Vec<i32>) -> Vec<BusinessProfileEntity> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::Id.is_in(ids))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    /// Text search over `business_name`/`social_name`, optionally narrowed to a
    /// single `business_type` (e.g. "Professional"). Powers marketplace/
    /// professional discovery alongside the location filter in
    /// `BusinessProfileAddressGateway::find_all_within_radius_of_point`.
    pub async fn search_by_query(
        db: &DbConn,
        query: &str,
        business_type: Option<&str>,
        limit: u64,
    ) -> Vec<i32> {
        let search_pattern = format!("%{}%", escape_like(query));
        let mut condition = Condition::any()
            .add(entity::business_profile_entity::Column::BusinessName.ilike(&search_pattern))
            .add(entity::business_profile_entity::Column::SocialName.ilike(&search_pattern));

        let mut find = BusinessProfileQuery::find().filter(condition.clone());
        if let Some(business_type) = business_type {
            condition = Condition::all()
                .add(condition)
                .add(entity::business_profile_entity::Column::BusinessType.eq(business_type));
            find = BusinessProfileQuery::find().filter(condition);
        }

        find.order_by_asc(entity::business_profile_entity::Column::Id)
            .limit(limit)
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(|p| p.id)
            .collect()
    }

    /// Every business profile of a given `business_type`, for discovery
    /// requests supplying only a location filter (no text query).
    pub async fn find_all_ids_by_business_type(db: &DbConn, business_type: &str) -> Vec<i32> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile_entity::Column::BusinessType.eq(business_type))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(|p| p.id)
            .collect()
    }

    /// Deletes a single business profile, as part of an account-purge cascade —
    /// its address/team-member/profile rows must be deleted first (plain FKs).
    pub async fn delete_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<DeleteResult, DbErr> {
        BusinessProfileQuery::delete_by_id(id).exec(db).await
    }
}
