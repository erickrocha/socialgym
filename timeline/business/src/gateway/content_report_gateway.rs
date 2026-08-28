use domain::business_error::BusinessError;
use domain::content_report::ContentReport;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Collection, Database};

pub struct ContentReportGateway {
    collection: Collection<ContentReport>,
}
impl ContentReportGateway {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("content_reports"),
        }
    }
    pub async fn persist(&self, report: ContentReport) -> Result<ContentReport, BusinessError> {
        self.collection
            .insert_one(&report)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?;
        Ok(report)
    }
    pub async fn find(&self, id: &str) -> Result<Option<ContentReport>, BusinessError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }
    pub async fn list(&self, status: Option<&str>) -> Result<Vec<ContentReport>, BusinessError> {
        let filter = status.map(|s| doc! { "status": s }).unwrap_or_default();
        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! { "priority": 1, "createdAt": 1 })
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?;
        let mut rows = vec![];
        while let Some(row) = cursor
            .try_next()
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?
        {
            rows.push(row);
        }
        Ok(rows)
    }
    pub async fn replace(&self, report: &ContentReport) -> Result<(), BusinessError> {
        self.collection
            .replace_one(doc! { "_id": &report.uuid }, report)
            .await
            .map(|_| ())
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }
    pub async fn delete_for_reporter(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .delete_many(doc! { "reporterPersonUuid": person_uuid })
            .await
            .map(|_| ())
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }
}
