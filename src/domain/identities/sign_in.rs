use crate::io::event_store::types::Event;
use crate::io::jwt::Jwt;
use crate::io::password;
use crate::io::result::{ClientError, Error};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::EventData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Response {
    pub jwt: String,
}

pub async fn handler(db: PgPool, jwt: Jwt, args: Args) -> Result<Response, Error> {
    let events = sqlx::query_as::<_, Event<EventData>>(
        "select * from identities.events where data->>'email' = $1 order by sequence_num asc",
    )
    .bind(args.email.clone())
    .fetch_all(&db)
    .await?;

    let identity = events
        .iter()
        .find(|&event| event.data.email == args.email)
        .ok_or(Error::BadRequest(ClientError::new(
            "AUTHENTICATION_FAILED",
            "Wrong email or password.",
        )))?;

    let verify_result = password::verify(&identity.data.password_hash, &args.password)?;

    match verify_result {
        true => {
            let encoded_jwt = jwt.encode(
                &identity.stream_id.to_string(),
                &identity.data.role,
                &args.email.clone(),
            )?;
            Ok(Response { jwt: encoded_jwt })
        }
        false => Err(Error::BadRequest(ClientError::new(
            "AUTHENTICATION_FAILED",
            "Wrong email or password.",
        ))),
    }
}

pub async fn controller(
    web_db: web::Data<PgPool>,
    web_jwt: web::Data<Jwt>,
    web_args: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let result = handler(
        web_db.get_ref().clone(),
        web_jwt.get_ref().clone(),
        web_args.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
