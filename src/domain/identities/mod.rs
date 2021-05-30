use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
pub mod list;
pub mod sign_up;
// pub mod show;
// pub mod sign_in;

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
                "/sign_up",
                web::post().to(
                    |web_db: web::Data<PgPool>, web_args: web::Json<sign_up::Args>| {
                        sign_up::handler(web_db.get_ref().clone(), web_args.into_inner())
                    },
                ),
            ),
    );
}
