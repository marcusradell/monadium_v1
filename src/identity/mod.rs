use crate::db;
use crate::models;
use crate::schema::identity;
use crate::schema::identity::dsl;
use actix_web::{web, Error, HttpResponse, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde;

pub fn create_post<'a>(
    conn: &PgConnection,
    email: &'a str,
    password_hash: &'a str,
) -> models::Identity {
    let new_identity = models::NewIdentity {
        email,
        password_hash,
    };

    diesel::insert_into(identity::table)
        .values(&new_identity)
        .get_result(conn)
        .expect("Error saving new identity.")
}

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
            .service(web::resource("/sign_up").route(web::post().to(sign_up)))
            .service(web::resource("/list").route(web::get().to(list))),
    );
}
