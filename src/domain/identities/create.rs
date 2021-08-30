use super::{EventData, EventMeta};
use crate::io::error::Error;
use crate::io::jwt::Jwt;
use crate::io::password;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler(args: Args, role: String, db: PgPool, jwt: Jwt) -> Result<String, Error> {
    let password_hash = password::hash(&args.password)?;
    let data = Json(EventData {
        email: args.email.clone(),
        password_hash,
        role,
    });

    let meta = Json(EventMeta {
        cid: Uuid::new_v4(),
    });

    let id = Uuid::new_v4();

    sqlx::query!(
        r#"
insert into events
(stream_id, version, type, data, meta) VALUES
( $1, $2, $3, $4, $5 )
returning sequence_num
        "#,
        id,
        1,
        "IDENTITIES/SIGNED_UP",
        data as _,
        meta as _
    )
    .fetch_one(&db)
    .await?;

    let result = jwt.encode(id.to_string(), args.email)?;

    Ok(result)
}

pub async fn controller(
    args: web::Json<Args>,
    db: web::Data<PgPool>,
    jwt: web::Data<Jwt>,
) -> Result<HttpResponse, Error> {
    let owner_email =
        std::env::var("IDENTITIES_OWNER_EMAIL").expect("Missing IDENTITIES_OWNER_EMAIL.");

    let owner_password =
        std::env::var("IDENTITIES_OWNER_PASSWORD").expect("Missing IDENTITIES_OWNER_PASSWORD.");

    let args = args.into_inner();

    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let result = handler(
        args,
        role.into(),
        db.get_ref().clone(),
        jwt.get_ref().clone(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
