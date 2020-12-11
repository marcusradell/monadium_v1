use actix_web::{post, HttpResponse, Responder};
use std::fmt;

pub struct Err {
    code: &'static str,
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"code\": {}", self.code)
    }
}
pub struct SignInWithEmailResult {
    ok: bool,
    error: Err,
}

impl fmt::Display for SignInWithEmailResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"ok\": {}, \"error\": {}}}", self.ok, self.error)
    }
}

#[post("/identity")]
pub async fn controller() -> impl Responder {
    let result = SignInWithEmailResult {
        ok: false,
        error: Err { code: "failed" },
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(result.to_string())
}
