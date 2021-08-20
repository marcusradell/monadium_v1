use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use serde::{Deserialize,Serialize};

use crate::io::error::Error;

#[derive(Debug, Serialize)]
pub struct Profile {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    id: String
}

async fn handler(query: Query, _db: PgPool)->Result<Profile,Error>{
    let result = Profile{
        id: query.id,
        name: "TODO".into()
    };

    Ok(result)
}

pub async fn controller(query: web::Path<Query>, db: web::Data<PgPool>)->Result<HttpResponse, Error>{
    let result=handler(query.into_inner(),db.get_ref().clone()).await?;
    Ok(HttpResponse::Ok().json(result))
}