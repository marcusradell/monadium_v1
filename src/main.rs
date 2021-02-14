#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate dotenv;
use dotenv::dotenv;
mod app;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = io::db::init();
    let jwt = io::jwt::Jwt::new(String::from("todo_set_as_env_var"));

    io::http::init(
        db_pool,
        jwt,
        String::from("0.0.0.0:8080"),
        vec![app::health::configure, app::identity::configure],
    )
    .await
}
