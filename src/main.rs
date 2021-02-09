#[macro_use]
extern crate diesel;
extern crate dotenv;
mod db;
mod health;
mod http;
mod identity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::init();
    http::init(db_pool).await
}
