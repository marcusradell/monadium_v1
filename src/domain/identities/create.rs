use super::{sign_in, EventData};
use crate::io::jwt::Jwt;
use crate::io::password;
use crate::io::result::Error;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

const EVENT_TYPE: &str = "IDENTITIES/CREATED";

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler(
    args: Args,
    role: String,
    db: PgPool,
    jwt: Jwt,
) -> Result<sign_in::Response, Error> {
    let existing_identity = sqlx::query!(
        r#"select * from identities.events where event_type = $1 and data->>'email' = $2 limit 1"#,
        EVENT_TYPE,
        args.email.clone()
    )
    .fetch_optional(&db)
    .await?;

    match existing_identity {
        Some(_) => {
            return sign_in::handler(
                db,
                jwt,
                sign_in::Args {
                    email: args.email.clone(),
                    password: args.password,
                },
            )
            .await
        }
        None => {
            let password_hash = password::hash(&args.password)?;
            let data = EventData {
                email: args.email.clone(),
                password_hash,
                role: role.clone(),
            };

            let cid = Uuid::new_v4();

            let id = Uuid::new_v4();

            // TODO: Inject it. Then trait it (scary!).
            let repo = super::repo::Repo::new(&db);
            repo.create(id, data, cid).await?;

            let result = sign_in::Response {
                jwt: jwt.encode(&id.to_string(), &role, &args.email)?,
            };

            Ok(result)
        }
    }
}

pub async fn controller(
    args: web::Json<Args>,
    db: web::Data<PgPool>,
    jwt: web::Data<Jwt>,
) -> Result<HttpResponse, Error> {
    let owner_email =
        std::env::var("IDENTITIES_OWNER_EMAIL").expect("Missing IDENTITIES_OWNER_EMAIL.");

    let owner_password =
        std::env::var("IDENTITIES_OWNER_PASSWORD").expect("Missing IDENTITIES_OWNER_PASSWORD.");

    let args = args.into_inner();

    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let result = handler(
        args,
        role.into(),
        db.get_ref().clone(),
        jwt.get_ref().clone(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
