use super::error::Error;
use actix_web::error::BlockingError;
use diesel::prelude::*;
use std::env;

use diesel::r2d2::{self, ConnectionManager, PoolError};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.")
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Error {
        Error::InternalServerError
    }
}

impl From<BlockingError<diesel::result::Error>> for Error {
    fn from(_error: BlockingError<diesel::result::Error>) -> Error {
        Error::InternalServerError
    }
}

impl From<r2d2::Error> for Error {
    fn from(_error: r2d2::Error) -> Error {
        Error::InternalServerError
    }
}
