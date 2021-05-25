use actix_web::web::{self, ServiceConfig};

mod new;
mod set_image;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("profile")
            .route("new", web::post().to(new::controller))
            .route("set_image", web::post().to(set_image::controller)),
    );
}
