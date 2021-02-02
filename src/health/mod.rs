use actix_web::{web, HttpResponse, Responder};

async fn live() -> impl Responder {
    HttpResponse::Ok()
}

async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .service(web::resource("/live").route(web::get().to(live)))
            .service(web::resource("/ready").route(web::get().to(ready))),
    );
}
