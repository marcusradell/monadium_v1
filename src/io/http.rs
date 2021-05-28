use crate::{domain::identities, io::jwt};
use actix_web::{middleware, web, App, HttpServer};

pub async fn init(
    jwt: jwt::Jwt,
    address: String,
    configure_list: Vec<fn(&mut web::ServiceConfig)>,
    identities_svc: identities::Service,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut scope = web::scope("");

        for configure in configure_list.clone() {
            scope = scope.configure(configure);
        }

        scope = scope.configure(|cfg| identities_svc.clone().config(cfg));

        App::new()
            .data(jwt.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(scope)
    })
    .bind(address)?
    .run();

    println!("Server started.");

    server.await
}
