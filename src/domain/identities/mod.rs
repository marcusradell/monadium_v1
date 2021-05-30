use std::sync::Arc;

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
        cfg.service(
            web::scope("/identities")
                .route(
                    "/list",
                    web::get().to({
                        let this = self.clone();
                        move |_: HttpRequest| this.clone().list()
                    }),
                )
                .route(
                    "/sign_up",
                    web::post().to({
                        let this = self.clone();
                        move |web_args: web::Json<sign_up::Args>| {
                            this.clone().sign_up(web_args.into_inner())
                        }
                    }),
                ),
        );
    }
}
