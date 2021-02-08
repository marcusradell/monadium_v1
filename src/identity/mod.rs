use crate::db;
use crate::models;
use crate::schema::identity;
use crate::schema::identity::dsl;
use actix_web::{web, Error, HttpResponse, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde;

#[derive(serde::Deserialize)]
pub struct SignUpArgs {
    email: String,
}

async fn sign_up(
    pool: web::Data<db::Pool>,
    args: web::Json<SignUpArgs>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    web::block(move || {
        diesel::insert_into(dsl::identity)
            .values(models::NewIdentity {
                email: &args.email,
                password_hash: "##password_hash##",
            })
            .execute(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

async fn list(pool: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let result = web::block(move || dsl::identity.load::<models::Identity>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .route("/sign_up", web::post().to(sign_up))
            .route("/list", web::get().to(list)),
    );
}
