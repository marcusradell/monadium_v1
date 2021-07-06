extern crate argon2;
extern crate dotenv;

use dotenv::dotenv;
mod domain;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Booting server.");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let jwt = io::jwt::Jwt::new();

    let uri = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL.");

    let db = io::db::new(uri).await.unwrap();

    let http = io::http::init(
        8080,
        jwt,
        db,
        vec![
            domain::health::config,
            domain::profile::config,
            domain::identities::config,
        ],
    )
    .await?;

    http.server.await
}
