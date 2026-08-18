use async_trait::async_trait;
use domain::business_error::BusinessError;
#[async_trait]
pub trait Repository<T, ID>: Send + Sync
where
    T: Send,
    ID: Send,
{
    async fn persist(&self, entity: T) -> Result<T, BusinessError>;
    async fn find_by_id(&self, id: ID) -> Option<T>;
    async fn update(&self, id: ID, entity: T) -> Result<T, BusinessError>;
    async fn delete(&self, id: ID) -> Result<bool, BusinessError>;
}
