use actix_web::web::{self, ServiceConfig};
use serde::Serialize;

mod new;
mod set_image;
mod show;

#[derive(Debug, Serialize)]
pub struct Profile {
    id: String,
    status: Status,
    name: String,
    date_of_birth: String,
    phone_number: String,
    email: String,
    location: String 
}

#[derive(Debug, Serialize)]
pub enum Status {
    Active,
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("profiles")
            .route("new", web::post().to(new::controller))
            .route("set_image", web::post().to(set_image::controller))
            .route("show/{id}", web::get().to(show::controller))
    );
}
