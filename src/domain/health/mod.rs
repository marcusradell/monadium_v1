use actix_web::{web, HttpResponse, Responder};

async fn live() -> impl Responder {
    HttpResponse::Ok()
}

async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(live));
    cfg.service(
        web::scope("/health")
            .route("/live", web::get().to(live))
            .route("/ready", web::get().to(ready)),
    );
}
