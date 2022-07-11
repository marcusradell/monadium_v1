extern crate argon2;
extern crate dotenv;

use actix_web::{web, HttpServer};
use dotenv::dotenv;
mod domain;
mod event;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    dev_api::tracing::init("monadiun".to_string()).expect("Failed to initialize tracer.");
    // All logs should be wrapped in a span. This is automatically done for each controller.
    tracing::info_span!("main:server_starting").in_scope(|| {
        tracing::info!("Starting server...");
    });

    let jwt = dev_api::jwt::Jwt::new(dev_api::ensure_env("JWT_SECRET").as_bytes());

    let db = dev_api::db::init_pg().await;

    let identities_repo = domain::identities::repo::Repo::new(&db);

    let server = HttpServer::new(move || {
        let configs: Vec<fn(&mut web::ServiceConfig)> = vec![
            domain::health::config,
            domain::identities::config,
            domain::profiles::config,
        ];

        let app = dev_api::http::new(configs);

        // Extend the app with your own dependencies.
        app.app_data(web::Data::new(jwt.clone()))
            .app_data(web::Data::new(identities_repo.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    tracing::info_span!("main:server_started").in_scope(|| {
        tracing::info!("Server started!");
    });

    server.await
}
