mod controller;
mod test;
use super::types::CreatedEvent;
use crate::domain::identities::repo::types::RepoList;
use crate::io::result::Error;
pub use controller::controller;

pub async fn handler(repo: &mut impl RepoList) -> Result<Vec<CreatedEvent>, Error> {
    repo.list().await
}
