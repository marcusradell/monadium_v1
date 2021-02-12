use super::models;
use super::schema::identity::dsl;
use crate::io::db;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Args {
    email: String,
}

pub async fn show(pool: web::Data<db::Pool>, args: web::Query<Args>) -> Result<HttpResponse> {
    use super::schema::identity::dsl::*;
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let result = web::block(move || {
        dsl::identity
            .filter(email.eq(args.email.clone()))
            .first::<models::Identity>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(result))
}
