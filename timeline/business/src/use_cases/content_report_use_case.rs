use crate::gateway::content_report_gateway::ContentReportGateway;
use crate::gateway::post_gateway::PostGateway;
use crate::gateway::role_gateway::RoleGateway;
use crate::repositories::repository::Repository;
use domain::business_error::BusinessError;
use domain::content_report::{ContentReport, ModerationEvent};
use mongodb::Database;
use mongodb::bson::DateTime;

pub struct ContentReportUseCase;
impl ContentReportUseCase {
    pub async fn create(
        db: &Database,
        reporter: &str,
        target_type: String,
        target_id: String,
        post_id: String,
        reason: String,
        details: Option<String>,
    ) -> Result<ContentReport, BusinessError> {
        if !matches!(target_type.as_str(), "post" | "comment" | "media")
            || target_id.is_empty()
            || post_id.is_empty()
            || reason.is_empty()
        {
            return Err(BusinessError::validation("invalid content report"));
        }
        if PostGateway::new(db)
            .find_by_id(post_id.clone())
            .await
            .is_none()
        {
            return Err(BusinessError::not_found("post not found"));
        }
        ContentReportGateway::new(db)
            .persist(ContentReport::new(
                target_type,
                target_id,
                post_id,
                reporter.to_string(),
                reason,
                details,
            ))
            .await
    }
    pub async fn list(
        db: &Database,
        status: Option<&str>,
    ) -> Result<Vec<ContentReport>, BusinessError> {
        RoleGateway::require("moderator").await?;
        ContentReportGateway::new(db).list(status).await
    }
    pub async fn decide(
        db: &Database,
        id: &str,
        moderator: &str,
        decision: &str,
        reason: &str,
    ) -> Result<ContentReport, BusinessError> {
        RoleGateway::require("moderator").await?;
        if !matches!(decision, "removed" | "dismissed") || reason.trim().is_empty() {
            return Err(BusinessError::validation("invalid moderation decision"));
        }
        let mut report = ContentReportGateway::new(db)
            .find(id)
            .await?
            .ok_or_else(|| BusinessError::not_found("report not found"))?;
        if decision == "removed" {
            let posts = PostGateway::new(db);
            match report.target_type.as_str() {
                "post" => {
                    posts.delete(report.post_id.clone()).await?;
                }
                "comment" => {
                    posts
                        .remove_comment_for_moderation(&report.post_id, &report.target_id)
                        .await?
                }
                "media" => {
                    posts
                        .remove_media_for_moderation(&report.post_id, &report.target_id)
                        .await?
                }
                _ => return Err(BusinessError::validation("invalid report target")),
            }
        }
        report.status = "resolved".into();
        report.decision = Some(decision.into());
        report.removal_reason = Some(reason.into());
        report.assigned_moderator_uuid = Some(moderator.into());
        report.updated_at = DateTime::now();
        report.history.push(ModerationEvent {
            actor_person_uuid: moderator.into(),
            action: decision.into(),
            reason: reason.into(),
            created_at: DateTime::now(),
        });
        ContentReportGateway::new(db).replace(&report).await?;
        Ok(report)
    }
}
