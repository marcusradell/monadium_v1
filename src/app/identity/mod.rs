use actix_web::web;
mod list;
mod models;
mod new;
mod schema;
mod sign_in;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .route("/new", web::post().to(new::new))
            .route("/list", web::get().to(list::list))
            .route("/sign_in", web::post().to(sign_in::sign_in)),
    );
}
