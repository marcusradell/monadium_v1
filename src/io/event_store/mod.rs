use crate::io::result::{Error, Result};
use sqlx::postgres::PgPoolOptions;

use self::types::EventStorer;

pub mod mock;
pub mod types;

#[derive(Clone)]
pub struct EventStore {
    db: sqlx::PgPool,
}

impl EventStore {
    pub async fn new(uri: String) -> Result<Self> {
        let db = PgPoolOptions::new().connect(&uri).await?;

        Ok(Self { db })
    }

    /// Only used as an escape hatch until the API is more stable. Should be deprecated in the future.
    pub fn db(&self) -> &sqlx::PgPool {
        &self.db
    }
}

impl<T: Clone> EventStorer<T> for EventStore {
    fn add(&mut self, _event: types::Event<T>) -> Result<()> {
        todo!()
    }

    fn list(&self) -> Result<Vec<types::Event<T>>> {
        todo!()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{:?}", error);
        Error::InternalServerError
    }
}
