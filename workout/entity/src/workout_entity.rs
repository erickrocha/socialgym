use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;

pub type WorkoutEntity = Model;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workout")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub owner_id: i32,
    pub owner_uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub muscle_group: String,
    pub visibility: String,
    pub status: String,
    pub assigned_by_profile_id: Option<i32>,
    pub assigned_by_profile_uuid: Option<Uuid>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person_entity::Entity",
        from = "Column::OwnerId",
        to = "super::person_entity::Column::Id"
    )]
    Person,
    #[sea_orm(has_many = "super::workout_exercise_entity::Entity")]
    WorkoutExercise,
}

impl Related<super::person_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::workout_exercise_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkoutExercise.def()
    }
}

// Many-to-many relation to Exercise via WorkoutExercise
impl Related<super::exercise_entity::Entity> for Entity {
    fn to() -> RelationDef {
        super::workout_exercise_entity::Relation::Exercise.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::workout_exercise_entity::Relation::Workout.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.uuid = Set(Uuid::new_v4());
            self.created_at = Set(chrono::Utc::now());
        }
        self.updated_at = Set(chrono::Utc::now());
        Ok(self)
    }
}
