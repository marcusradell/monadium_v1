use crate::app::health;
use crate::app::identity;
use crate::io::db;
use actix_web::{middleware, web, App, HttpServer};

pub async fn init(db_pool: db::Pool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(health::configure)
                    .configure(identity::configure),
            )
    })
    .bind("0.0.0.0:8080")?
    .run();

    println!("Server started.");

    Ok(())
}
