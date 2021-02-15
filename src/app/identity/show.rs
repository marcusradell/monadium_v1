use super::model;
use crate::io::db;
use crate::schema::identity::dsl::*;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Args {
    email: String,
}

pub async fn show(pool: web::Data<db::Pool>, args: web::Query<Args>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let result = web::block(move || {
        identity
            .filter(email.eq(args.email.clone()))
            .first::<model::Identity>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(result))
}
