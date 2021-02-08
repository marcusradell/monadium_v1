use actix_web::web;
mod list;
pub mod models;
mod new;
pub mod schema;

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .route("/new", web::post().to(new::new))
            .route("/list", web::get().to(list::list)),
    );
}
