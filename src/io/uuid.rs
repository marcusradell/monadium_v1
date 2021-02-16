use super::errors::ServiceError;

impl From<uuid::Error> for ServiceError {
    fn from(_: uuid::Error) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}
