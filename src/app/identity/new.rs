use super::model;
use crate::io::db;
use crate::schema::identity::dsl;
use actix_web::{web, Error, HttpResponse};
use argon2::Config;
use diesel::prelude::*;
use rand::Rng;

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

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    let password_hash =
        argon2::hash_encoded(args.password.as_bytes(), &salt, &config).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    web::block(move || {
        diesel::insert_into(dsl::identity)
            .values(model::NewIdentity {
                email: &args.email,
                password_hash: &password_hash,
                created_at: chrono::Local::now().naive_local(),
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
