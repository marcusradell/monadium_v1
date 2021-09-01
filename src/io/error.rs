use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display)]

pub enum Error {
    InternalServerError,
    BadRequest(ClientError),
}

#[derive(Debug, Display, Serialize)]
#[display(fmt = "[{}]:{}", code, message)]
pub struct ClientError {
    message: String,
    code: String,
}

impl ClientError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError().json(ClientError {
                code: "INTERNAL_ERROR".into(),
                message: "Thoughts and prayers.".into(),
            }),
            Error::BadRequest(client_error) => HttpResponse::BadRequest().json(client_error),
        }
    }
}
