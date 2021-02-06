use crate::models::{Identity, NewIdentity};
use crate::schema::identity;
use actix_web::{web, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde;

pub fn create_post<'a>(conn: &PgConnection, email: &'a str, password_hash: &'a str) -> Identity {
    let new_identity = NewIdentity {
        email,
        password_hash,
    };

    diesel::insert_into(identity::table)
        .values(&new_identity)
        .get_result(conn)
        .expect("Error saving new identity.")
}

#[derive(serde::Deserialize)]
pub struct SignUpArgs {
    email: String,
}

pub async fn sign_up(args: web::Json<SignUpArgs>) -> Result<&'static str> {
    Ok("TODO")
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity").service(web::resource("/sign_up").route(web::post().to(sign_up))),
    );
}
