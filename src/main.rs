#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::{middleware, web, App, HttpServer};
mod db;
mod health;
mod identity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::connect();

    let s = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(identity::schema)
                    .configure(health::schema),
            )
    })
    .bind("0.0.0.0:8080")?
    .run();

    println!("Server started.");

    s.await
}
