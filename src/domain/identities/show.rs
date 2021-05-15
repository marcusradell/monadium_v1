use super::model;
use crate::io::db;
use crate::io::error::Error;
use crate::schema::identity::dsl::*;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Args {
    email: String,
}

pub async fn show(
    pool: web::Data<db::Pool>,
    args: web::Query<Args>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get()?;

    let result = web::block(move || {
        identity
            .filter(email.eq(args.email.clone()))
            .first::<model::Identity>(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
