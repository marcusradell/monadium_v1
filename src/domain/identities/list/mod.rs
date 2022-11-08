mod controller;
use super::types::CreatedEvent;
use crate::domain::identities::repo::types::RepoList;
pub use controller::controller;
use dev_api::Error;

pub async fn handler(repo: &mut impl RepoList) -> Result<Vec<CreatedEvent>, Error> {
    repo.list().await
}
