use super::model;
use crate::io::db;
use crate::io::password;
use crate::schema::identity::dsl;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct SignUpArgs {
    pub email: String,
    pub password: String,
}

pub async fn new(
    pool: web::Data<db::Pool>,
    args: web::Json<SignUpArgs>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let password_hash = password::hash(&args.password)?;

    web::block(move || {
        diesel::insert_into(dsl::identity)
            .values(model::NewIdentity {
                email: &args.email,
                password_hash: &password_hash,
                created_at: chrono::Local::now().naive_local(),
            })
            .execute(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}
