use super::models;
use super::schema::identity::dsl;
use crate::db;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct SignUpArgs {
    pub email: String,
}

pub async fn new(
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
