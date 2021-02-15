use super::model;
use crate::io::db;
use crate::schema::identity::dsl;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

pub async fn list(pool: web::Data<db::Pool>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let result = web::block(move || dsl::identity.load::<model::Identity>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}
