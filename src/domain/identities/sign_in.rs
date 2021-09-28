use super::repo::{types::RepoFindByEmail, Repo};
use crate::io::jwt::Jwt;
use crate::io::password;
use crate::io::result::{ClientError, Error};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Response {
    pub jwt: String,
}

pub async fn handler(
    repo: &mut impl RepoFindByEmail,
    jwt: Jwt,
    now: i64,
    args: Args,
) -> Result<Response, Error> {
    let identity = repo
        .find_by_email(&args.email)
        .await?
        .ok_or(Error::BadRequest(ClientError::new(
            "NOT_FOUND",
            &format!("Could not find an identity with email {}", args.email),
        )))?;

    let verify_result = password::verify(&identity.data.password_hash, &args.password)?;

    match verify_result {
        false => Err(Error::BadRequest(ClientError::new(
            "AUTHENTICATION_FAILED",
            "Wrong email or password.",
        ))),
        true => {
            let encoded_jwt = jwt.encode(
                &identity.stream_id,
                &identity.data.role,
                &args.email.clone(),
                now,
            )?;
            Ok(Response { jwt: encoded_jwt })
        }
    }
}

pub async fn controller(
    web_repo: web::Data<Repo>,
    web_jwt: web::Data<Jwt>,
    web_args: web::Json<Args>,
) -> Result<HttpResponse, Error> {
    let result = handler(
        &mut web_repo.get_ref().clone(),
        web_jwt.get_ref().clone(),
        Utc::now().timestamp(),
        web_args.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::io::jwt::Claims;

    use super::super::repo::mock::RepoMock;
    use super::*;

    #[actix_rt::test]
    async fn not_found() {
        let mut repo = RepoMock::new();
        let jwt = Jwt::from_secret("secret");
        let now = Utc::now().timestamp();

        let result = handler(
            &mut repo,
            jwt,
            now,
            Args {
                email: "email@example.com".into(),
                password: "password".into(),
            },
        )
        .await
        .unwrap_err();

        assert_eq!(
            result,
            Error::BadRequest(ClientError::new(
                "NOT_FOUND",
                "Could not find an identity with email email@example.com"
            ))
        )
    }

    #[actix_rt::test]
    async fn authentication_failed() {
        let mut repo = RepoMock::new();
        let jwt = Jwt::from_secret("supersecret");
        let now = Utc::now().timestamp();

        let result = handler(
            &mut repo,
            jwt,
            now,
            Args {
                email: "existing_user@example.com".into(),
                password: "failedpassword".into(),
            },
        )
        .await
        .unwrap_err();

        assert_eq!(repo.result(), &vec!["email: existing_user@example.com"]);

        assert_eq!(
            result,
            Error::BadRequest(ClientError::new(
                "AUTHENTICATION_FAILED",
                "Wrong email or password."
            ))
        )
    }

    #[actix_rt::test]
    async fn signed_in() {
        let mut repo = RepoMock::new();
        let jwt = Jwt::from_secret("pillutadig");
        let now = Utc::now().timestamp();

        let response = handler(
            &mut repo,
            jwt.clone(),
            now,
            Args {
                email: "existing_user@example.com".into(),
                password: "password".into(),
            },
        )
        .await
        .unwrap();

        let result = jwt.decode(response.jwt).unwrap();

        assert_eq!(repo.result(), &vec!["email: existing_user@example.com"]);

        assert_eq!(
            result,
            Claims {
                id: Uuid::from_u128(1),
                role: "MEMBER".into(),
                email: "existing_user@example.com".into(),
                exp: now + 15 * 60
            }
        );
    }
}
