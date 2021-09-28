use std::str::FromStr;

use crate::io::{
    event_store::types::Event,
    password,
    result::{Error, Result},
};
use async_trait::async_trait;
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use super::CreatedData;

const EVENT_TYPE: &str = "IDENTITIES/CREATED";

type CreatedEvent = Event<CreatedData>;

#[async_trait]
pub trait RepoCreate {
    // TODO: Do I need to make self &mut when the real implementation only needs &self?
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()>;
}

#[async_trait]
pub trait RepoFindByEmail {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<Event<CreatedData>>>;
}

pub struct RepoMock {
    result: Vec<String>,
    data: Vec<Event<CreatedData>>,
}

impl RepoMock {
    pub fn new() -> Self {
        RepoMock {
            result: vec![],
            data: vec![CreatedEvent {
                stream_id: Uuid::from_u128(1),
                sequence_num: 1,
                version: 1,
                cid: Uuid::from_u128(2),
                event_type: "IDENTITY/CREATED".into(),
                inserted_at: "2021-01-31T23:59:30Z".parse().unwrap(),
                data: Json(CreatedData {
                    email: "existing_user@example.com".into(),
                    password_hash: password::hash("password_hash").unwrap().into(),
                    role: "MEMBER".into(),
                }),
            }],
        }
    }

    pub fn result(&self) -> &Vec<String> {
        &self.result
    }
}

#[async_trait]
impl RepoCreate for RepoMock {
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()> {
        self.result
            .push(format!("id: {:?}, data: {:?}, cid: {:?}", id, data, cid));

        Ok(())
    }
}

#[async_trait]
impl RepoFindByEmail for RepoMock {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<Event<CreatedData>>> {
        self.result.push(format!("email: {}", email));

        if email == self.data[0].data.email {
            Ok(Some(self.data[0].clone()))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct Repo {
    db: PgPool,
}

#[async_trait]
impl RepoCreate for Repo {
    async fn create(&mut self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()> {
        let data = Json(data);

        sqlx::query!(
            r#"
    insert into identities.events
    (stream_id, version, event_type, data, cid) VALUES
    ( $1, $2, $3, $4, $5 )
    returning sequence_num
            "#,
            id,
            1,
            EVENT_TYPE,
            data as _,
            cid
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl RepoFindByEmail for Repo {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<Event<CreatedData>>> {
        Ok(sqlx::query_as!(
            FindByEmailResult,
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
            data->>'email' = $1
            order by sequence_num asc"#,
            email
        )
        .fetch_optional(&self.db)
        .await?)
    }
}

impl Repo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn exists_by_email(&self, email: &str) -> Result<Option<()>> {
        let result = sqlx::query!(
            r#"select * from identities.events where event_type = $1 and data->>'email' = $2 limit 1"#,
            EVENT_TYPE,
            email.clone()
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(result.and(Some(())))
    }

    pub async fn show(&self, id: &Uuid) -> Result<Event<CreatedData>> {
        sqlx::query_as!(
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
            id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(Error::InternalServerError)
    }
}

type FindByEmailResult = Event<CreatedData>;
