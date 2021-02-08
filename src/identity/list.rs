use super::models;
use super::schema::identity::dsl;
use crate::db;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

pub async fn list(pool: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let result = web::block(move || dsl::identity.load::<models::Identity>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}
