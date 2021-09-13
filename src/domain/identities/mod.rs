use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod create;
pub mod list;
pub mod repo;
pub mod show;
pub mod sign_in;
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct EventData {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identities")
            .route("/list", web::get().to(list::controller))
            .route("/show/{id}", web::get().to(show::controller))
            .route("/sign_in", web::post().to(sign_in::controller))
            .route("/create", web::post().to(create::controller)),
    );
}
