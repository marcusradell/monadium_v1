use crate::io::jwt::Jwt;
use actix_web::{dev::Server, middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub struct Http {
    pub port: u16,
    pub server: Server,
}

pub async fn init(
    port_or_zero: u16,
    jwt: Jwt,
    db: PgPool,
    configs: Vec<fn(&mut web::ServiceConfig)>,
) -> std::io::Result<Http> {
    let listener: TcpListener = TcpListener::bind(("0.0.0.0", port_or_zero))?;
    let port = listener.local_addr()?.port();

    let server = HttpServer::new(move || {
        let mut scope = web::scope("");
        for config in configs.clone() {
            scope = scope.configure(config);
        }

        App::new()
            .data(jwt.clone())
            .data(db.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(scope)
    })
    .listen(listener)?
    .run();

    println!("Server started.");

    Ok(Http { port, server })
}
