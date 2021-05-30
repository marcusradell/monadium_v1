use crate::io::jwt::Jwt;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;

pub async fn init(
    address: String,
    jwt: Jwt,
    db: PgPool,
    configs: Vec<fn(&mut web::ServiceConfig)>,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut scope = web::scope("");
        for config in configs.clone() {
            scope = scope.configure(config);
        }

        App::new()
            .app_data(jwt.clone())
            .app_data(db.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(scope)
    })
    .bind(address)?
    .run();

    println!("Server started.");

    server.await
}
