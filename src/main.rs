#[macro_use]
extern crate diesel;
extern crate dotenv;
mod app;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = io::db::init();
    io::http::init(db_pool).await
}
