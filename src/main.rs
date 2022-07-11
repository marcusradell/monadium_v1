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
            .app_data(web::Data::new(identities_repo))
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    tracing::info_span!("main:server_started").in_scope(|| {
        tracing::info!("Server started!");
    });

    server.await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     println!("Booting server.");
//     dotenv().ok();

//     std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
//     env_logger::init();

//     let jwt = io::jwt::Jwt::new();

//     let uri = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL.");

//     // TODO: Deprecate or merge with how the identities_repo work now.
//     let es = io::event_store::EventStore::new(&uri).await.unwrap();
//     let db = es.db().clone();
//     // TODO: This should replace the EventStore.
//     let identities_repo = domain::identities::repo::Repo::new(&db);

//     let deps_experiment = crate::domain::deps_experiment::Deps {
//         fake_db: "db".into(),
//         fake_mq: "mq".into(),
//     };

//     let http = io::http::init(
//         8080,
//         jwt,
//         db,
//         identities_repo,
//         vec![
//             domain::health::config,
//             domain::profiles::config,
//             domain::identities::config,
//         ],
//         deps_experiment,
//     )
//     .await?;

//     http.server.await
// }
