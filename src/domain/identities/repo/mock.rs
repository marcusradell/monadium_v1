#![allow(dead_code)]

use super::{
    super::types::{CreatedData, CreatedEvent},
    types::{RepoCreate, RepoFindByEmail},
};
use crate::io::password;
use crate::io::result::Result;
use async_trait::async_trait;
use sqlx::types::Json;
use uuid::Uuid;
pub struct RepoMock {
    result: Vec<String>,
    data: Vec<CreatedEvent>,
}

impl RepoMock {
    pub fn new() -> Self {
        RepoMock {
            result: vec![],
            data: vec![CreatedEvent {
                stream_id: Uuid::from_u128(1),
                sequence_num: 1,
                version: 1,
                cid: Uuid::from_u128(2),
                event_type: "IDENTITY/CREATED".into(),
                inserted_at: "2021-01-31T23:59:30Z".parse().unwrap(),
                data: Json(CreatedData {
                    email: "existing_user@example.com".into(),
                    password_hash: password::hash_mock("password").unwrap().into(),
                    role: "MEMBER".into(),
                }),
            }],
        }
    }

    pub fn result(&self) -> &Vec<String> {
        &self.result
    }
}

#[async_trait]
impl RepoCreate for RepoMock {
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()> {
        self.result
            .push(format!("id: {:?}, data: {:?}, cid: {:?}", id, data, cid));

        Ok(())
    }
}

#[async_trait]
impl RepoFindByEmail for RepoMock {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<CreatedEvent>> {
        self.result.push(format!("email: {}", email));

        if email == self.data[0].data.email {
            Ok(Some(self.data[0].clone()))
        } else {
            Ok(None)
        }
    }
}
