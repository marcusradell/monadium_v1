#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate dotenv;
use dotenv::dotenv;
mod app;
mod io;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let db_pool = io::db::init();

    let jwt = io::jwt::Jwt::new();

    io::http::init(
        db_pool,
        jwt,
        "0.0.0.0:8080".into(),
        vec![app::health::configure, app::identity::configure],
    )
    .await
}
