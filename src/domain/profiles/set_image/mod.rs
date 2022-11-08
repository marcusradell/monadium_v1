use actix_multipart::Multipart;
use actix_web::HttpResponse;
use dev_api::{Error, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
    id: String,
    image: String,
}
#[derive(Debug, Serialize)]
pub struct Event {
    id: String,
    image: String,
}

async fn _handler(cmd: Command) -> Result<()> {
    dbg!(&cmd);

    let event = Event {
        id: cmd.id,
        image: cmd.image,
    };

    dbg!(&event);

    // TODO: store event in the db.

    Ok(())
}

pub async fn controller(mut multipart: Multipart) -> Result<HttpResponse> {
    while let Some(item) = multipart.next().await {
        let mut field = item.map_err(|e| {
            tracing::error!("{:?}", e);
            Error::internal_error()
        })?;

        while let Some(chunk) = field.next().await {
            println!(
                "-- CHUNK: \n{:?}",
                std::str::from_utf8(&chunk.map_err(|e| {
                    tracing::error!("{:?}", e);
                    Error::internal_error()
                })?)
            );
        }
    }

    // let result = handler(cmd).await?;
    // Ok(HttpResponse::Ok().json(result))
    Ok(HttpResponse::Ok().finish())
}
