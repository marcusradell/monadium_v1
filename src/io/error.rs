use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error; please try again later."),
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
