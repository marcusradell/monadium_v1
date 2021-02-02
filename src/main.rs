#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
mod health;
mod identity;
mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Identity, NewIdentity};

pub fn create_post<'a>(conn: &PgConnection, email: &'a str, password_hash: &'a str) -> Identity {
    use schema::identity;

    let new_identity = NewIdentity {
        email,
        password_hash,
    };

    diesel::insert_into(identity::table)
        .values(&new_identity)
        .get_result(conn)
        .expect("Error saving new identity.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::schema::identity::dsl;

    let connection = establish_connection();
    println!("DB connection established.");

    // create_post(&connection, "me@example.com", "##password_hash##");

    let result = dsl::identity
        .limit(5)
        .load::<models::Identity>(&connection)
        .expect("Error loading idetities.");

    println!("Displaying {} identities.", result.len());

    for row in result {
        println!("{}", row.id);
        println!("{}", row.email);
        println!("{}", row.password_hash);
    }

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
