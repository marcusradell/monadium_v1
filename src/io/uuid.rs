use super::error::{ClientError, Error};

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Error {
        // TODO: Don't assume too much. Find out what can actually go wrong.
        Error::BadRequest(ClientError{code:"INVALID_UUID".into(),message: "Something went wrong while handling a UUID. That's all we got for you at this point. Move along.".into()})
    }
}
