use crate::m20260129_000007_create_table_workout::Workout;
use crate::m20260129_000008_create_table_exercise::Exercise;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the junction table workout_exercise
        manager
            .create_table(
                Table::create()
                    .table(WorkoutExercise::Table)
                    .if_not_exists()
                    .col(pk_auto(WorkoutExercise::Id))
                    .col(uuid_uniq(WorkoutExercise::Uuid))
                    .col(integer(WorkoutExercise::WorkoutId).not_null())
                    .col(integer(WorkoutExercise::ExerciseId).not_null())
                    .col(integer(WorkoutExercise::OrderIndex).not_null().default(0))
                    .col(timestamp_with_time_zone(WorkoutExercise::CreatedAt).null())
                    .col(timestamp_with_time_zone(WorkoutExercise::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        // Create foreign key to workout
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_workout_exercise_workout")
                    .from_tbl(WorkoutExercise::Table)
                    .from_col(WorkoutExercise::WorkoutId)
                    .to_tbl(Workout::Table)
                    .to_col(Workout::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create foreign key to exercise
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_workout_exercise_exercise")
                    .from_tbl(WorkoutExercise::Table)
                    .from_col(WorkoutExercise::ExerciseId)
                    .to_tbl(Exercise::Table)
                    .to_col(Exercise::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create unique index on workout_id + exercise_id
        manager
            .create_index(
                Index::create()
                    .name("idx_workout_exercise_unique")
                    .table(WorkoutExercise::Table)
                    .col(WorkoutExercise::WorkoutId)
                    .col(WorkoutExercise::ExerciseId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_workout_exercise_unique")
                    .table(WorkoutExercise::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_workout_exercise_workout")
                    .table(WorkoutExercise::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_workout_exercise_exercise")
                    .table(WorkoutExercise::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the junction table
        manager
            .drop_table(Table::drop().table(WorkoutExercise::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum WorkoutExercise {
    Table,
    Id,
    Uuid,
    WorkoutId,
    ExerciseId,
    OrderIndex,
    CreatedAt,
    UpdatedAt,
}
