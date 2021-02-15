use super::model;
use crate::io::db;
use crate::io::jwt;
use crate::schema::identity::dsl::*;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SignInArgs {
    email: String,
    password: String,
}

#[derive(serde::Serialize, Debug)]
struct SignInResponse {
    jwt: String,
}

pub async fn sign_in(
    pool: web::Data<db::Pool>,
    jwt: web::Data<jwt::Jwt>,
    args: web::Json<SignInArgs>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get DB connection from pool.");

    let cloned_args = args.clone();
    let identity_by_email = web::block(move || {
        identity
            .filter(email.eq(cloned_args.email))
            .first::<model::Identity>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let verify_result =
        argon2::verify_encoded(&identity_by_email.password_hash, args.password.as_bytes())
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    match verify_result {
        true => {
            let encoded_jwt = jwt.into_inner().encode(args.email.clone()).map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
            Ok(HttpResponse::Ok().json(SignInResponse { jwt: encoded_jwt }))
        }
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}
