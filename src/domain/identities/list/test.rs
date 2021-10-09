#![cfg(test)]

use crate::domain::identities::{list::handler, repo::mock::RepoMock};

#[actix_rt::test]
async fn list_of_one() {
    let mut repo = RepoMock::new();
    let (created, _) = repo.member_created();
    let result = handler(&mut repo).await;

    assert_eq!(result, Ok(vec![created]))
}
