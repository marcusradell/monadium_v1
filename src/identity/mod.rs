use actix_web::{web, HttpResponse, Responder};
use std::fmt;

pub struct Err {
    code: &'static str,
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"code\": {}", self.code)
    }
}
pub struct SignUpWithEmailResult {
    ok: bool,
    error: Err,
}

impl fmt::Display for SignUpWithEmailResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"ok\": {}, \"error\": {}}}", self.ok, self.error)
    }
}

pub async fn sign_up() -> impl Responder {
    let result = SignUpWithEmailResult {
        ok: false,
        error: Err { code: "failed" },
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(result.to_string())
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity").service(web::resource("/sign_up").route(web::post().to(sign_up))),
    );
}
