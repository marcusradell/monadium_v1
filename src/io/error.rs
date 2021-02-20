use actix_web::HttpResponse;
use actix_web::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error; please try again later."),
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized => HttpResponse::Unauthorized()
                .json("Unauthorized; please sign in with an authorized account."),
        }
    }
}
