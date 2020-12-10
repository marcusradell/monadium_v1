use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

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

#[post("/api/identity")]
async fn identity_controller() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"ok\": false, \"error\": {\"code\": \"failed\"}}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .service(identity_controller)
            .service(hello)
            .service(live)
            .service(ready)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
