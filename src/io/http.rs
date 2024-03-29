use crate::{domain::{deps_experiment, identities}, io::jwt::Jwt};
use actix_web::{App, HttpRequest, HttpServer, dev::Server, http::header, middleware, web};
use sqlx::PgPool;
use std::net::TcpListener;
use super::result::{ClientError, Error};

pub struct Http {
    pub port: u16,
    pub server: Server,
}

pub async fn init(
    port_or_zero: u16,
    jwt: Jwt,
    db: PgPool,
    identities_repo: identities::repo::Repo,
    configs: Vec<fn(&mut web::ServiceConfig)>,
    deps_experiment: deps_experiment::Deps
) -> std::io::Result<Http> {
    let listener: TcpListener = TcpListener::bind(("0.0.0.0", port_or_zero))?;
    let port = listener.local_addr()?.port();

    let server = HttpServer::new(move || {
        let mut scope = web::scope("");
        for config in configs.clone() {
            scope = scope.configure(config);
        }



        scope = scope.configure( |cfg|deps_experiment.config(cfg));

        App::new()
            .data(jwt.clone())
            .data(db.clone())
            .data(identities_repo.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(scope)
    })
    .listen(listener)?
    .run();

    println!("Server started.");

    Ok(Http { port, server })
}

pub fn jwt_from(req: HttpRequest) -> Result<String, Error> {
    let result = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(Error::BadRequest(ClientError::bad_request("Missing the Authorization header. Check the value and try again.")))?
        .to_str()
        .map_err(|_| 
            Error::BadRequest(ClientError::bad_request(
                "Failed to parse the Authorization header as an ASCII string. Check the value and try again."))
        )?
        .split_once("Bearer ").map(|(_, bearer_token)| {
bearer_token
        }).ok_or(Error::BadRequest(ClientError::bad_request( "Couldn't split the Authorization value on 'Bearer '. Did you forget to add 'Bearer ' in front of the JWT?")))?;

    Ok(result.into())
}
