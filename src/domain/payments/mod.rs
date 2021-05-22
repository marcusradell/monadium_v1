use actix_web::web::{self, ServiceConfig};
mod notify;
mod pay;

pub fn config(c: &mut ServiceConfig) {
    c.service(
        web::scope("/payments")
            .route("/pay", web::post().to(pay::controller))
            .route("/notify", web::post().to(notify::controller)),
    );
}
