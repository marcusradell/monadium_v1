use crate::io::error::Error;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Command {
    name: String,
}
#[derive(Debug, Serialize)]
pub struct Event {
    id: String,
    name: String,
}

async fn handler(cmd: Command) -> Result<(), Error> {
    dbg!(&cmd);

    let event = Event {
        id: Uuid::new_v4().to_string(),
        name: cmd.name,
    };

    dbg!(&event);

    // TODO: store event in the db.

    Ok(())
}

pub async fn controller(cmd: web::Json<Command>) -> Result<HttpResponse, Error> {
    let result = handler(cmd.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}
