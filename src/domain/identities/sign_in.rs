use super::Event;
use crate::io::error::Error;
use crate::io::jwt::Jwt;
use crate::io::password;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    email: String,
    password: String,
}

#[derive(serde::Serialize, Debug)]
struct Response {
    jwt: String,
}

pub async fn handler(db: PgPool, jwt: Jwt, args: Args) -> Result<HttpResponse, Error> {
    let events = sqlx::query_as::<_, Event>(
        "select * from events where data->>'email' = $1 order by sequence_num asc",
    )
    .bind(args.email.clone())
    .fetch_all(&db)
    .await?;

    let identity = events
        .iter()
        .find(|&event| event.data.email == args.email)
        .ok_or(Error::BadRequest("Wrong email or password.".into()))?;

    let verify_result = password::verify(&identity.data.password_hash, &args.password)?;

    match verify_result {
        true => {
            let encoded_jwt = jwt.encode(args.email.clone())?;
            Ok(HttpResponse::Ok().json(Response { jwt: encoded_jwt }))
        }
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}
