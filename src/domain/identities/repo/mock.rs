#![allow(dead_code)]

use super::{
    super::types::{CreatedData, CreatedEvent},
    types::{RepoCreate, RepoFindByEmail},
};
use crate::io::result::Result;
use async_trait::async_trait;
use std::convert::TryInto;
use uuid::Uuid;

pub struct RepoMock {
    data: Vec<CreatedEvent>,
}

impl RepoMock {
    pub fn new() -> Self {
        RepoMock { data: vec![] }
    }

    pub fn insert_fixture(&mut self, event: CreatedEvent) {
        self.data.push(event);
    }

    pub fn data(&self) -> &Vec<CreatedEvent> {
        &self.data
    }
}

#[async_trait]
impl RepoCreate for RepoMock {
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()> {
        self.data.push(CreatedEvent::new(
            id,
            (self.data.len() + 1).try_into().unwrap(),
            data,
            cid,
        ));

        Ok(())
    }
}

#[async_trait]
impl RepoFindByEmail for RepoMock {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<CreatedEvent>> {
        Ok(self
            .data
            .clone()
            .into_iter()
            .find(|item| item.data.email == email))
    }
}
