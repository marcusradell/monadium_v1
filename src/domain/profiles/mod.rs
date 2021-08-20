use actix_web::web::{self, ServiceConfig};

mod new;
mod set_image;
mod show;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("profiles")
            .route("new", web::post().to(new::controller))
            .route("set_image", web::post().to(set_image::controller))
            .route("show/{id}", web::get().to(show::controller))
    );
}
