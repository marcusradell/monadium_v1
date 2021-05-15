use super::model;
use crate::io::db;
use crate::io::error::Error;
use crate::io::jwt;
use crate::io::password;
use crate::schema::identity::dsl::*;
use actix_web::{web, HttpResponse};
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
    let conn = pool.get()?;

    let cloned_args = args.clone();
    let identity_by_email = web::block(move || {
        identity
            .filter(email.eq(cloned_args.email))
            .first::<model::Identity>(&conn)
    })
    .await?;

    let verify_result = password::verify(&identity_by_email.password_hash, &args.password)?;

    match verify_result {
        true => {
            let encoded_jwt = jwt.into_inner().encode(args.email.clone())?;
            Ok(HttpResponse::Ok().json(SignInResponse { jwt: encoded_jwt }))
        }
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}
