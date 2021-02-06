#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::{web, App, HttpServer};
mod db;
mod health;
mod identity;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = db::establish_connection();
    db::seed(&connection);

    println!("DB connection established.");

    let s = HttpServer::new(|| {
        App::new().service(
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
