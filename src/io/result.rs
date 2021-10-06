use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, PartialEq)]
pub enum Error {
    InternalServerError,
    BadRequest(ClientError),
}

#[derive(Debug, Display, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    NotFound,
    InternalError,
    AuthFailed,
    AuthTokenExpired,
    AccessDenied,
    BadRequest,
}

#[derive(Debug, Display, Serialize, PartialEq)]
#[display(fmt = "[{}]:{}", code, message)]
pub struct ClientError {
    message: String,
    code: ErrorCode,
}

impl ClientError {
    pub fn access_denied(actual_role: &str, expected_role: &str) -> Self {
        Self {
            code: ErrorCode::AccessDenied,
            message: format!(
                "Found role {}, but expected {}.",
                actual_role, expected_role
            ),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self {
            code: ErrorCode::BadRequest,
            message: message.into(),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            code: ErrorCode::NotFound,
            message: format!("Could not find {}.", id),
        }
    }

    pub fn authentication_failed() -> Self {
        Self {
            code: ErrorCode::AuthFailed,
            message: "Wrong email or password.".into(),
        }
    }

    pub fn authentication_expired() -> Self {
        Self {
            code: ErrorCode::AuthTokenExpired,
            message: "Your JWT has expired. Please sign in again to receive and new one.".into(),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: ErrorCode::InternalError,
            message: "Thoughts and prayers.".into(),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json(ClientError::internal_error())
            }
            Error::BadRequest(client_error) => HttpResponse::BadRequest().json(client_error),
        }
    }
}
