use super::model;
use crate::io::db;
use crate::io::error::Error;
use crate::schema::identity::dsl;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn list(pool: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get()?;
    let result = web::block(move || dsl::identity.load::<model::Identity>(&conn)).await?;
    Ok(HttpResponse::Ok().json(result))
}
