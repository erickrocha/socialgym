use crate::commons::entity_mapper::EntityMapper;
use crate::domain::enums::InviteStatus;
use crate::domain::friend::{Friend, FriendEntityMapper};
use entity::friends_entity as friends;
use entity::friends_entity::Column;
use entity::prelude::FriendsEntity as FriendsQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr, DeleteResult,
    EntityTrait, QueryFilter,
};
use crate::commons::functions::parse_uuid;

pub struct FriendGateway {}

impl FriendGateway {
    pub async fn persist(db: &DbConn, friend: Friend) -> Result<friends::ActiveModel, DbErr> {
        let active_model = FriendEntityMapper::build_active_model(friend);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, friend: Friend) -> Result<friends::FriendsEntity, DbErr> {
        let active_model = FriendEntityMapper::build_active_model(friend);
        active_model.update(db).await
    }

    pub async fn find_friend_request(
        db: &DbConn,
        sender_id: i32,
        receiver_id: i32,
    ) -> Result<Option<friends::FriendsEntity>, DbErr> {
        FriendsQuery::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(friends::Column::PersonId.eq(sender_id))
                            .add(friends::Column::FriendId.eq(receiver_id)),
                    )
                    .add(
                        Condition::all()
                            .add(friends::Column::PersonId.eq(receiver_id))
                            .add(friends::Column::FriendId.eq(sender_id)),
                    ),
            )
            .one(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<friends::FriendsEntity>, DbErr> {
        FriendsQuery::find_by_id(id).one(db).await
    }

    pub async fn find_all_by_column_and_status(
        db: &DbConn,
        column: Column,
        person_id: i32,
        status: InviteStatus,
    ) -> Result<Vec<friends::FriendsEntity>, DbErr> {
        FriendsQuery::find()
            .filter(column.eq(person_id))
            .filter(friends::Column::Status.eq(status.as_str().to_string()))
            .all(db)
            .await
    }

    pub async fn find_all_accepted_friends(
        db: &DbConn,
        person_id: i32,
    ) -> Result<Vec<friends::FriendsEntity>, DbErr> {
        FriendsQuery::find()
            .filter(
                Condition::any()
                    .add(Column::PersonId.eq(person_id))
                    .add(Column::FriendId.eq(person_id)),
            )
            .filter(Column::Status.eq(InviteStatus::Accepted.as_str()))
            .all(db)
            .await
    }

    pub async fn find_all_accepted_friends_by_uuid(
        db: &DbConn,
        person_uuid: String,
    ) -> Result<Vec<friends::FriendsEntity>, DbErr> {
        let person_uuid = parse_uuid(&person_uuid).map_err(|e| DbErr::Type(e.to_string()))?;
        FriendsQuery::find()
            .filter(
                Condition::any()
                    .add(Column::PersonUuid.eq(person_uuid))
                    .add(Column::FriendUuid.eq(person_uuid)),
            )
            .filter(Column::Status.eq(InviteStatus::Accepted.as_str()))
            .all(db)
            .await
    }

    pub async fn find_all_related_person_ids(
        db: &DbConn,
        person_id: i32,
    ) -> Result<Vec<i32>, DbErr> {
        let records = FriendsQuery::find()
            .filter(
                Condition::any()
                    .add(Column::PersonId.eq(person_id))
                    .add(Column::FriendId.eq(person_id)),
            )
            .all(db)
            .await?;

        let related_ids: Vec<i32> = records
            .into_iter()
            .map(|f| {
                if f.person_id == person_id {
                    f.friend_id
                } else {
                    f.person_id
                }
            })
            .collect();

        Ok(related_ids)
    }

    /// Bulk-deletes every friendship row involving a person, on either side of
    /// the relationship (account-purge cascade).
    pub async fn delete_all_involving_person<C: ConnectionTrait>(db: &C, person_id: i32) -> Result<DeleteResult, DbErr> {
        FriendsQuery::delete_many()
            .filter(
                Condition::any()
                    .add(Column::PersonId.eq(person_id))
                    .add(Column::FriendId.eq(person_id)),
            )
            .exec(db)
            .await
    }
}
