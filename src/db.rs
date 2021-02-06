use crate::models;
use crate::schema::identity::dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn seed(connection: &PgConnection) {
    // identity::create_post(&connection, "me@example.com", "##password_hash##");

    let result = dsl::identity
        .limit(5)
        .load::<models::Identity>(connection)
        .expect("Error loading idetities.");

    println!("Displaying {} identities.", result.len());

    for row in result {
        println!("{}", row.id);
        println!("{}", row.email);
        println!("{}", row.password_hash);
    }
}
