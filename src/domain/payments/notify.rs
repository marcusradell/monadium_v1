use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Args {}

pub async fn controller(_body: web::Json<Args>) -> Result<HttpResponse> {
    todo!();
}
