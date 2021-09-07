extern crate argon2;
extern crate dotenv;

use dotenv::dotenv;
mod domain;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Booting server.");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let jwt = io::jwt::Jwt::new();

    let uri = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL.");

    let event_store = io::event_store::EventStore::new(&uri).await.unwrap();

    let http = io::http::init(
        8080,
        jwt,
        event_store,
        vec![
            domain::health::config,
            domain::profiles::config,
            domain::identities::config,
        ],
    )
    .await?;

    http.server.await
}
