use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "exercise")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i32,
    pub owner_uuid: Uuid,
    pub owner_name: String,
    pub sets: i32,
    #[sea_orm(name = "exercise_type")]
    pub category: String,
    pub reps_or_duration: i32,
    pub visibility: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::workout_exercise::Entity")]
    WorkoutExercise,
}

impl Related<super::workout_exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkoutExercise.def()
    }
}

// Many-to-many relation to Workout via WorkoutExercise
impl Related<super::workout::Entity> for Entity {
    fn to() -> RelationDef {
        super::workout_exercise::Relation::Workout.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::workout_exercise::Relation::Exercise.def().rev())
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
