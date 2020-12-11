use actix_web::{web, HttpResponse, Responder};
use serde;

#[derive(serde::Serialize)]
pub struct Err {
    code: &'static str,
}

#[derive(serde::Serialize)]
pub struct SignUpWithEmailResult {
    ok: bool,
    error: Err,
}

pub async fn sign_up() -> impl Responder {
    let result = SignUpWithEmailResult {
        ok: false,
        error: Err { code: "failed" },
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity").service(web::resource("/sign_up").route(web::post().to(sign_up))),
    );
}
