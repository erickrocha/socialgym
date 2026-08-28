use crate::gateway::content_report_gateway::ContentReportGateway;
use crate::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use crate::gateway::mention_notification_gateway::MentionNotificationGateway;
use crate::gateway::post_gateway::PostGateway;
use crate::gateway::workout_session_gateway::WorkoutSessionGateway;
use domain::business_error::BusinessError;
use mongodb::Database;

pub struct AccountDataDeletionUseCase {}

impl AccountDataDeletionUseCase {
    /// Cascade-deletes every piece of data belonging to `person_uuid`: their
    /// own posts (and embedded comments/reactions on them), their
    /// comments/reactions left on other people's posts, evolution check-ins,
    /// workout-session mirrors, and mention/notification records.
    ///
    /// Each step is independently idempotent (deleting/pulling already-gone
    /// data is a no-op), so the caller (workout's purge sweep) can safely
    /// retry the whole call if any single step fails partway through.
    pub async fn delete_all_for_person(
        db: &Database,
        person_uuid: &str,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting all timeline data for person_uuid={}", person_uuid);

        PostGateway::new(db)
            .pull_reactions_by_author(person_uuid)
            .await?;
        PostGateway::new(db)
            .pull_comments_by_author(person_uuid)
            .await?;
        PostGateway::new(db)
            .delete_all_by_author(person_uuid)
            .await?;
        EvolutionCheckInGateway::new(db)
            .delete_all_by_person(person_uuid)
            .await?;
        WorkoutSessionGateway::new(db)
            .delete_all_by_person(person_uuid)
            .await?;
        MentionNotificationGateway::new(db)
            .delete_all_involving_person(person_uuid)
            .await?;
        ContentReportGateway::new(db)
            .delete_for_reporter(person_uuid)
            .await?;

        Ok(())
    }
}
