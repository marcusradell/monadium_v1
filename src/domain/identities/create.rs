use super::{
    repo::{
        types::{RepoCreate, RepoFindByEmail},
        Repo,
    },
    sign_in,
    types::CreatedData,
};
use crate::io::jwt::Jwt;
use crate::io::password;
use crate::io::result::Error;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler(
    args: Args,
    owner_email: &str,
    owner_password: &str,
    cid: Uuid,
    jwt: Jwt,
    repo: &mut (impl RepoCreate + RepoFindByEmail),
    now: i64,
    id: Uuid,
) -> Result<sign_in::Response, Error> {
    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let exists = repo.find_by_email(&args.email).await?;

    match exists {
        // Email found, try signing them in instead of creating a new identity.
        Some(_) => {
            return sign_in::handler(
                repo,
                jwt,
                now,
                sign_in::Args {
                    email: args.email.clone(),
                    password: args.password,
                },
            )
            .await;
        }
        None => {
            let password_hash = password::hash(&args.password)?;
            let data = CreatedData {
                email: args.email.clone(),
                password_hash,
                role: role.to_string(),
            };
            repo.create(id, data, cid).await?;
            let result = sign_in::Response {
                jwt: jwt.encode(&id, &role, &args.email, now)?,
            };

            Ok(result)
        }
    }
}

pub async fn controller(
    args: web::Json<Args>,
    jwt: web::Data<Jwt>,
    repo: web::Data<Repo>,
) -> Result<HttpResponse, Error> {
    let owner_email =
        std::env::var("IDENTITIES_OWNER_EMAIL").expect("Missing IDENTITIES_OWNER_EMAIL.");

    let owner_password =
        std::env::var("IDENTITIES_OWNER_PASSWORD").expect("Missing IDENTITIES_OWNER_PASSWORD.");

    let args = args.into_inner();
    let now = Utc::now().timestamp();
    let id = Uuid::new_v4();
    let cid = Uuid::new_v4();

    let result = handler(
        args,
        &owner_email,
        &owner_password,
        cid,
        jwt.get_ref().clone(),
        &mut repo.get_ref().clone(),
        now,
        id,
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[cfg(test)]
mod test {
    use crate::{
        domain::identities::{repo::mock::RepoMock, types::CreatedEvent},
        io::jwt::{Claims, Jwt},
    };

    use super::*;

    #[actix_rt::test]
    async fn sign_in_existing_member() {
        let jwt = Jwt::from_secret("jwt_secret");
        let mut repo = RepoMock::new();
        let now = Utc::now().timestamp();

        let handler_result = handler(
            Args {
                email: "existing_user@example.com".into(),
                password: "password".into(),
            },
            "nomatch@example.com".into(),
            "password".into(),
            Uuid::from_u128(2),
            jwt.clone(),
            &mut repo,
            now,
            Uuid::from_u128(999999999),
        )
        .await
        .unwrap();

        let result = jwt.decode(handler_result.jwt).unwrap();

        assert_eq!(
            result,
            Claims {
                email: "existing_user@example.com".into(),
                id: Uuid::from_u128(1),
                exp: now + 15 * 60,
                role: "MEMBER".into()
            }
        )
    }

    #[actix_rt::test]
    async fn create_member() {
        let jwt = Jwt::from_secret("sake_is_better_than_whiskey");
        let mut repo = RepoMock::new();
        let now = Utc::now().timestamp();
        let id = Uuid::from_u128(100);

        let result = handler(
            Args {
                email: "created@example.com".into(),
                password: "pass".into(),
            },
            "no_match_here@example.com",
            "coffee_latte",
            Uuid::from_u128(2),
            jwt.clone(),
            &mut repo,
            now,
            id,
        )
        .await
        .and_then(|response| jwt.decode(response.jwt));

        assert_eq!(
            result,
            Ok(Claims {
                email: "created@example.com".into(),
                id,
                exp: now + 15 * 60,
                role: "MEMBER".into()
            })
        )
    }

    #[actix_rt::test]
    async fn create_owner() {
        let jwt = Jwt::from_secret("my_kids_will_never_get_to_play_hockey");
        let mut repo = RepoMock::new();
        let now = Utc::now().timestamp();
        let id = Uuid::from_u128(100);

        let result = handler(
            Args {
                email: "created_owner@example.com".into(),
                password: "000".into(),
            },
            "created_owner@example.com",
            "000",
            Uuid::from_u128(2),
            jwt.clone(),
            &mut repo,
            now,
            id,
        )
        .await
        .and_then(|response| jwt.decode(response.jwt));

        assert_eq!(
            repo.data(),
            &vec![CreatedEvent::new(
                Uuid::from_u128(100),
                1,
                CreatedData {
                    email: "created_owner@example.com".into(),
                    password_hash: password::hash_mock("000").unwrap().into(),
                    role: "MEMBER".into(),
                },
                Uuid::from_u128(2),
            )]
        );

        assert_eq!(
            result,
            Ok(Claims {
                email: "created_owner@example.com".into(),
                id,
                exp: now + 15 * 60,
                role: "OWNER".into()
            })
        )
    }
}
