use super::result::{ClientError, Error};

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Error {
        // TODO: Don't assume too much. Find out what can actually go wrong.
        Error::BadRequest(ClientError::new("INVALID_UUID", "Something went wrong while handling a UUID. That's all we got for you at this point. Move along."))
    }
}
