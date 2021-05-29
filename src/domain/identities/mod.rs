use actix_web::{web, HttpRequest};
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

#[derive(Debug, Clone)]
pub struct Service {
    db: PgPool,
}

impl Service {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub fn config(self, cfg: &mut web::ServiceConfig) {
        let self_1 = self.clone();
        let self_2 = self.clone();

        cfg.service(
            web::scope("/identities")
                .route(
                    "/list",
                    web::get().to(move |_: HttpRequest| self_1.clone().list()),
                )
                .route(
                    "/sign_up",
                    web::post().to(move |web_args: web::Json<sign_up::Args>| {
                        self_2.clone().sign_up(web_args.into_inner())
                    }),
                ),
        );
    }
}
