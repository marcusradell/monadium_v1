use crate::io::db;
use actix_web::{middleware, web, App, HttpServer};

pub async fn init(
    db_pool: db::Pool,
    configure_list: Vec<fn(&mut web::ServiceConfig)>,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut scope = web::scope("/api");

        for configure in configure_list.clone() {
            scope = scope.configure(configure);
        }
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .service(scope)
    })
    .bind("0.0.0.0:8080")?
    .run();

    println!("Server started.");

    server.await
}