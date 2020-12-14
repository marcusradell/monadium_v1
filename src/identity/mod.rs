use actix_web::{web, Result};
use serde;

#[derive(serde::Deserialize)]
pub struct SignUpBody {
    email: String,
}

pub async fn sign_up(args: web::Json<SignUpBody>) -> Result<&'static str> {
    println!("sign_up called with email: {}", args.email);
    Ok("TODO")
}

pub fn schema(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity").service(web::resource("/sign_up").route(web::post().to(sign_up))),
    );
}
