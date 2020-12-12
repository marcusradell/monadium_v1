use actix_web::{web, HttpResponse, Responder};

async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

async fn live() -> impl Responder {
    HttpResponse::Ok()
}

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<H1>Hello Monadium!</H2>")
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .service(web::resource("/status").route(web::get().to(status)))
            .service(web::resource("/live").route(web::get().to(live)))
            .service(web::resource("/ready").route(web::get().to(ready))),
    );
}
