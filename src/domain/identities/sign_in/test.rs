#![cfg(test)]

use super::handler;
use crate::io::jwt::Claims;
use crate::io::password::mock::{hash, verify};
use crate::io::result::{ClientError, Error};
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
    let email = "email@example.com";
    let password = "password";

    let result = handler(&mut repo, verify, jwt, now, email, password)
        .await
        .unwrap_err();

    assert_eq!(
        result,
        Error::BadRequest(ClientError::not_found("email@example.com"))
    )
}

#[actix_rt::test]
async fn authentication_failed() {
    let mut repo = RepoMock::new();
    let jwt = Jwt::from_secret("supersecret");
    let now = Utc::now();

    repo.insert_fixture(CreatedEvent::new(
        Uuid::from_u128(1),
        1,
        CreatedData::mock_member(),
        Uuid::from_u128(2),
        now,
    ));

    let result = handler(
        &mut repo,
        verify,
        jwt,
        now,
        "existing_member@example.com",
        "failedpassword",
    )
    .await
    .unwrap_err();

    assert_eq!(
        result,
        Error::BadRequest(ClientError::authentication_failed())
    )
}

#[actix_rt::test]
async fn signed_in() {
    let mut repo = RepoMock::new();
    let jwt = Jwt::from_secret("pillutadig");
    let now = Utc::now();

    repo.insert_fixture(CreatedEvent::new(
        Uuid::from_u128(1),
        1,
        CreatedData::mock_member(),
        Uuid::from_u128(2),
        now,
    ));

    let response = handler(
        &mut repo,
        verify,
        jwt.clone(),
        now,
        "existing_member@example.com",
        "correct_password",
    )
    .await
    .unwrap();

    let result = jwt.decode(response.jwt).unwrap();

    assert_eq!(
        result,
        Claims {
            id: Uuid::from_u128(1),
            role: "MEMBER".into(),
            email: "existing_member@example.com".into(),
            exp: now.timestamp() + 15 * 60
        }
    );
}
