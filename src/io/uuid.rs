use super::error::Error;

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Error {
        Error::BadRequest("Invalid UUID".into())
    }
}
