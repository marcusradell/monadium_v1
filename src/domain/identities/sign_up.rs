use super::{EventData, EventMeta};
use crate::io::error::Error;
use crate::io::password;
use actix_web::HttpResponse;
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
    pub role: String,
}

pub async fn handler(db: PgPool, args: Args) -> Result<HttpResponse, Error> {
    let password_hash = password::hash(&args.password)?;
    let data = Json(EventData {
        email: args.email,
        password_hash,
        role: args.role,
    });
    let meta = Json(EventMeta {
        cid: Uuid::new_v4(),
    });

    let _result = sqlx::query!(
        r#"
insert into events
(stream_id, version, type, data, meta) VALUES
( $1, $2, $3, $4, $5 )
returning sequence_num
        "#,
        Uuid::new_v4(),
        1,
        "identities/signed_up",
        data as _,
        meta as _
    )
    .fetch_one(&db)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

// pub async fn new_member(
//     pool: web::Data<db::Pool>,
//     args: web::Json<SignUpArgs>,
// ) -> Result<HttpResponse, Error> {
//     new(pool.get_ref().clone(), args.into_inner(), "MEMBER".into()).await
// }

// pub async fn new_administrator(
//     pool: web::Data<db::Pool>,
//     args: web::Json<SignUpArgs>,
// ) -> Result<HttpResponse, Error> {
//     new(
//         pool.get_ref().clone(),
//         args.into_inner(),
//         "ADMINISTRATOR".into(),
//     )
//     .await
// }
