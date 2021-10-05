#![cfg(test)]

use crate::domain::identities::sign_in::Args;
use crate::io::password::mock::PasswordMock;
use crate::io::password::types::PasswordHasher;
use crate::io::result::{ClientError, Error};
use crate::{domain::identities::sign_in::handler, io::jwt::Claims};
use crate::{
    domain::identities::{
        repo::mock::RepoMock,
        types::{CreatedData, CreatedEvent},
    },
    io::jwt::Jwt,
};
use chrono::Utc;
use uuid::Uuid;

#[actix_rt::test]
async fn not_found() {
    let mut repo = RepoMock::new();
    let jwt = Jwt::from_secret("secret");
    let now = Utc::now();

    let result = handler(
        &mut repo,
        PasswordMock {},
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
    let password = PasswordMock {};
    let jwt = Jwt::from_secret("supersecret");
    let now = Utc::now();

    repo.insert_fixture(CreatedEvent::new(
        Uuid::from_u128(1),
        1,
        CreatedData {
            email: "existing_user_wrong_pass@example.com".into(),
            password_hash: password.clone().hash("password123").unwrap(),
            role: "MEMBER".into(),
        },
        Uuid::from_u128(2),
        now,
    ));

    let result = handler(
        &mut repo,
        password,
        jwt,
        now,
        Args {
            email: "existing_user_wrong_pass@example.com".into(),
            password: "failedpassword".into(),
        },
    )
    .await
    .unwrap_err();

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
    let password = PasswordMock {};
    let jwt = Jwt::from_secret("pillutadig");
    let now = Utc::now();

    repo.insert_fixture(CreatedEvent::new(
        Uuid::from_u128(1),
        1,
        CreatedData {
            email: "existing_user@example.com".into(),
            password_hash: password.clone().hash("correct_password").unwrap(),
            role: "MEMBER".into(),
        },
        Uuid::from_u128(2),
        now,
    ));

    let response = handler(
        &mut repo,
        password,
        jwt.clone(),
        now,
        Args {
            email: "existing_user@example.com".into(),
            password: "correct_password".into(),
        },
    )
    .await
    .unwrap();

    let result = jwt.decode(response.jwt).unwrap();

    assert_eq!(
        result,
        Claims {
            id: Uuid::from_u128(1),
            role: "MEMBER".into(),
            email: "existing_user@example.com".into(),
            exp: now.timestamp() + 15 * 60
        }
    );
}