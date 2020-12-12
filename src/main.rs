mod health;
mod identity;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting.");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .configure(identity::schema)
                .configure(health::schema),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
