use actix_web::web;
mod list;
mod models;
mod new;
mod schema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .route("/new", web::post().to(new::new))
            .route("/list", web::get().to(list::list)),
    );
}
