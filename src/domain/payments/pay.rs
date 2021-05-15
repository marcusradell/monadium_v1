use actix_web::{HttpResponse, Result};

use crate::io::error::Error;

fn handler() -> Result<(), Error> {
    Ok(())
}

pub async fn controller() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
