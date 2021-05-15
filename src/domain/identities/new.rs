use super::model;
use crate::io::db;
use crate::io::error::Error;
use crate::io::password;
use crate::schema::identity::dsl;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct SignUpArgs {
    pub email: String,
    pub password: String,
}

async fn new(pool: db::Pool, args: SignUpArgs, role: String) -> Result<HttpResponse, Error> {
    let conn = pool.get()?;
    let password_hash = password::hash(&args.password)?;

    web::block(move || {
        diesel::insert_into(dsl::identity)
            .values(model::NewIdentity {
                email: &args.email,
                password_hash: &password_hash,
                role: &role,
                created_at: chrono::Local::now().naive_local(),
            })
            .execute(&conn)
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn new_member(
    pool: web::Data<db::Pool>,
    args: web::Json<SignUpArgs>,
) -> Result<HttpResponse, Error> {
    new(pool.get_ref().clone(), args.into_inner(), "MEMBER".into()).await
}

pub async fn new_administrator(
    pool: web::Data<db::Pool>,
    args: web::Json<SignUpArgs>,
) -> Result<HttpResponse, Error> {
    new(
        pool.get_ref().clone(),
        args.into_inner(),
        "ADMINISTRATOR".into(),
    )
    .await
}
