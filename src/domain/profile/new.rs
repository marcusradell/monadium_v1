use actix_web::{HttpResponse, Result};

pub async fn controller() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
