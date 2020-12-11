mod identity;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/ready")]
async fn ready() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/live")]
async fn live() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<H1>Hello Monadium!</H2>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(identity::controller))
            .service(hello)
            .service(live)
            .service(ready)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
