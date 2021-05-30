use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use crate::io::jwt::Jwt;
pub mod list;
pub mod sign_up;
// pub mod show;
pub mod sign_in;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Event {
    sequence_num: i64,
    stream_id: uuid::Uuid,
    version: i32,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    type_: String,
    data: Json<EventData>,
    meta: Json<EventMeta>,
    inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct EventData {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct EventMeta {
    pub cid: Uuid,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identities")
            .route(
                "/list",
                web::get().to(|web_db: web::Data<PgPool>| list::handler(web_db.get_ref().clone())),
            )
            .route(
                "/sign_in",
                web::post().to(
                    |web_db: web::Data<PgPool>,
                     web_jwt: web::Data<Jwt>,
                     web_args: web::Json<sign_in::Args>| {
                        sign_in::handler(
                            web_db.get_ref().clone(),
                            web_jwt.get_ref().clone(),
                            web_args.into_inner(),
                        )
                    },
                ),
            )
            .route(
                "/sign_up",
                web::post().to(
                    |web_db: web::Data<PgPool>, web_args: web::Json<sign_up::Args>| {
                        sign_up::handler(web_db.get_ref().clone(), web_args.into_inner())
                    },
                ),
            ),
    );
}
