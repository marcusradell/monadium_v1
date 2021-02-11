#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate dotenv;
mod app;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = io::db_old::init();
    let jwt = io::jwt::Jwt::new(String::from("todo_set_as_env_var"));

    io::http::init(
        db_pool,
        jwt,
        vec![app::health::configure, app::identity::configure],
    )
    .await
}
