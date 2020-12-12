use actix_web::{web, HttpResponse, Responder};
use serde;

#[derive(serde::Serialize)]
enum Status {
    Live,
    Ready,
}

async fn live() -> impl Responder {
    HttpResponse::Ok()
}

async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

async fn status() -> impl Responder {
    let status = Status::Ready;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&status).unwrap())
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .service(web::resource("/live").route(web::get().to(live)))
            .service(web::resource("/ready").route(web::get().to(ready)))
            .service(web::resource("/status").route(web::get().to(status))),
    );
}
