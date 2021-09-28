use super::super::types::{CreatedData, CreatedEvent};
use crate::io::result::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RepoCreate {
    // TODO: Do I need to make self &mut when the real implementation only needs &self?
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()>;
}

#[async_trait]
pub trait RepoFindByEmail {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<CreatedEvent>>;
}
