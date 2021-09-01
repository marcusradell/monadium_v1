use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::io::error::{ClientError, Error};

use super::{Profile, Status};

#[derive(Debug, Deserialize)]
pub struct Args {
    id: String,
}

async fn handler(args: Args, _db: PgPool) -> Result<Profile, Error> {
    // Temp solution before using a DB.
    let marcus_profile = Profile {
        id: "1".into(),
        name: "Marcus Rådell".into(),
        date_of_birth: "1982-03-03".into(),
        status: Status::Active,
        email: "marcus@radell.net".into(),
        phone_number: "+46725223325".into(),
        location: "Snickarvägen 27, 19730 Bro, Sweden".into(),
    };

    if args.id != marcus_profile.id {
        return Err(Error::BadRequest(ClientError::new(
            "NOT_FOUND",
            &format!("Could not find ID {}.", args.id),
        )));
    }

    let result = marcus_profile;

    Ok(result)
}

pub async fn controller(
    query: web::Path<Args>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let result = handler(query.into_inner(), db.get_ref().clone()).await?;
    Ok(HttpResponse::Ok().json(result))
}
