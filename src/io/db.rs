use super::errors::ServiceError;
use diesel::prelude::*;
use std::env;

use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.")
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> ServiceError {
        match error {
            diesel::result::Error::DatabaseError(kind, info) => {
                if let diesel::result::DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
