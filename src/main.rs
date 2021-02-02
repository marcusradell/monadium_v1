use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod health;
mod identity;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    establish_connection();
    println!("DB connection established.");

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
