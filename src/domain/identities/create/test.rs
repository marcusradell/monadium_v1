#![cfg(test)]

use super::Args;

mod test {
    use chrono::Utc;
    use dev_api::jwt::Jwt;
    use uuid::Uuid;

    use crate::domain::identities::{
        create::handler,
        repo::mock::RepoMock,
        types::{CreatedData, CreatedEvent},
    };

    use super::*;

    #[actix_rt::test]
    async fn sign_in_existing_member() {
        let jwt = Jwt::new("jwt_secret".as_bytes());
        let mut repo = RepoMock::new();
        let now = Utc::now();

        let (member_created, password) = repo.member_created();

        let handler_result = handler(
            Args {
                email: member_created.data.email.clone(),
                password,
            },
            "nomatch@example.com".into(),
            "password".into(),
            Uuid::from_u128(2),
            verify,
            hash,
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
                email: member_created.data.email.clone(),
                id: Uuid::from_u128(1),
                exp: now.timestamp() + 15 * 60,
                role: "MEMBER".into()
            }
        )
    }

    #[actix_rt::test]
    async fn create_member() {
        let jwt = Jwt::from_secret("sake_is_better_than_whiskey");
        let mut repo = RepoMock::new();
        let now = Utc::now();
        let id = Uuid::from_u128(100);

        let result = handler(
            Args {
                email: "created@example.com".into(),
                password: "pass".into(),
            },
            "no_match_here@example.com",
            "coffee_latte",
            Uuid::from_u128(2),
            verify,
            hash,
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
                exp: now.timestamp() + 15 * 60,
                role: "MEMBER".into()
            })
        )
    }

    #[actix_rt::test]
    async fn create_owner() {
        let mut repo = RepoMock::new();
        let jwt = Jwt::from_secret("my_kids_will_never_get_to_play_hockey");
        let now = Utc::now();
        let id = Uuid::from_u128(100);

        let result = handler(
            Args {
                email: "created_owner@example.com".into(),
                password: "000".into(),
            },
            "created_owner@example.com",
            "000",
            Uuid::from_u128(2),
            verify,
            hash,
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
                    password_hash: hash("000").unwrap().into(),
                    role: "OWNER".into(),
                },
                Uuid::from_u128(2),
                now
            )]
        );

        assert_eq!(
            result,
            Ok(Claims {
                email: "created_owner@example.com".into(),
                id,
                exp: now.timestamp() + 15 * 60,
                role: "OWNER".into()
            })
        )
    }
}
