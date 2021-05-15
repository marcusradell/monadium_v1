use actix_web::web::{self, ServiceConfig};
mod pay;

pub fn configure(c: &mut ServiceConfig) {
    c.service(web::scope("/payments").route("/pay", web::post().to(pay::controller)));
}
