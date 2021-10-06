use super::result::{ClientError, Error};

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Error {
        // TODO: Find out what can actually go wrong.
        Error::BadRequest(ClientError::internal_error())
    }
}
