use actix_web::web::{self, ServiceConfig};

mod new;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("profile").route("new", web::post().to(new::controller)));
}
