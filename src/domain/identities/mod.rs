use actix_web::web;

pub mod create;
// pub mod list;
pub mod repo;
pub mod show;
pub mod sign_in;
pub mod types;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identities")
            // .route("/list", web::get().to(list::controller))
            .route("/show/{id}", web::get().to(show::controller))
            .route("/sign_in", web::post().to(sign_in::controller))
            .route("/create", web::post().to(create::controller)),
    );
}
