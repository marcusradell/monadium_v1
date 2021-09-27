use super::CreatedData;
use crate::io::{
    event_store::types::Event,
    http,
    jwt::Jwt,
    result::{ClientError, Error},
};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Args {
    id: Uuid,
}

const EVENT_TYPE: &str = "IDENTITIES/CREATED";

type CreatedEvent = Event<CreatedData>;

#[derive(Serialize)]
pub struct Identity {
    id: Uuid,
    version: i32,
    created_at: DateTime<Utc>,
    email: String,
    role: String,
}

pub async fn handler(db: &PgPool, args: Args) -> Result<Identity, Error> {
    let result = sqlx::query_as!(
        CreatedEvent,
        r#"select
        stream_id,
        sequence_num,
        version,
        event_type,
        cid,
        inserted_at,
        data as "data: Json<CreatedData>"
        from identities.events
        where
        event_type = $1 and
        stream_id = $2
        limit 1"#,
        EVENT_TYPE,
        args.id.clone()
    )
    .fetch_optional(db)
    .await?
    .ok_or(Error::InternalServerError)?;

    Ok(Identity {
        id: result.stream_id,
        version: result.version,
        created_at: result.inserted_at,
        email: result.data.email.clone(),
        role: result.data.role.clone(),
    })
}

pub async fn controller(
    jwt: web::Data<Jwt>,
    req: HttpRequest,
    db: web::Data<PgPool>,
    query: web::Path<Args>,
) -> Result<HttpResponse, Error> {
    let bearer_token = http::jwt_from(req)?;

    let claims = jwt.decode(bearer_token)?;

    if claims.role != "OWNER" && claims.id != query.id {
        return Err(Error::BadRequest(ClientError::new(
            "ACCESS_DENIED",
            &format!(
                "identities/show requires the role OWNER or to own the data based on identity ID. Found; - role: {} - id: {}.",
                claims.role, claims.id
            ),
        )));
    }

    let result = handler(&db.get_ref().clone(), query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}
