use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

pub mod create;
pub mod list;
pub mod show;
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
            .route("/list", web::get().to(list::controller))
            .route(
                "/show",
                web::get().to(|web_db: web::Data<PgPool>| show::handler(web_db.get_ref().clone())),
            )
            .route("/sign_in", web::post().to(sign_in::controller))
            .route("/create", web::post().to(create::controller)),
    );
}
