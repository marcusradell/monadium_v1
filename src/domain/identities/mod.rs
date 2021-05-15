use actix_web::web;
pub mod list;
pub mod model;
pub mod new;
pub mod show;
pub mod sign_in;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identities")
            .route("/new_member", web::post().to(new::new_member))
            .route("/new_administrator", web::post().to(new::new_administrator))
            .route("/show", web::get().to(show::show))
            .route("/list", web::get().to(list::list))
            .route("/sign_in", web::post().to(sign_in::sign_in)),
    );
}
