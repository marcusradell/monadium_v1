use actix_web::{web, HttpRequest};
use sqlx::PgPool;
pub mod list;
// pub mod new;
// pub mod show;
// pub mod sign_in;

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
            .route("/list", web::get().to(move|_:HttpRequest| self.clone().list()))
                // .route("/show", web::get().to(show::show))
                // .route("/new_member", web::post().to(new::new_member))
                // .route("/new_administrator", web::post().to(new::new_administrator))
                // .route("/sign_in", web::post().to(sign_in::sign_in)),
        );
    }
}
