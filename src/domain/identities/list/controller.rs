use super::handler;
use crate::{
    domain::identities::repo::Repo,
    io::{
        http,
        jwt::Jwt,
        result::{ClientError, Error},
    },
};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn controller(
    repo: web::Data<Repo>,
    jwt: web::Data<Jwt>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let bearer_token = http::jwt_from(req)?;

    let claims = jwt.decode(bearer_token)?;

    if claims.role != "OWNER" {
        return Err(Error::BadRequest(ClientError::access_denied(
            &claims.role,
            "OWNER",
        )));
    }

    let result = handler(&mut repo.get_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(result))
}
