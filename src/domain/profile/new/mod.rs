use crate::io::error::Error;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewProfileCommand {
    name: String,
}
#[derive(Debug, Serialize)]
pub struct NewProfileEvent {
    id: String,
    name: String,
}

async fn handler(cmd: NewProfileCommand) -> Result<(), Error> {
    dbg!(&cmd);

    let event = NewProfileEvent {
        id: Uuid::new_v4().to_string(),
        name: cmd.name,
    };

    dbg!(&event);

    // TODO: store event in the db.

    Ok(())
}

pub async fn controller(cmd: web::Json<NewProfileCommand>) -> Result<HttpResponse, Error> {
    let result = handler(cmd.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}
