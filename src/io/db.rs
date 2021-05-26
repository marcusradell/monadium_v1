use crate::io::error::Error;
use sqlx::postgres::PgPoolOptions;

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{:?}", error);
        Error::InternalServerError
    }
}

pub async fn new(uri: String) -> Result<sqlx::PgPool, Error> {
    let result = PgPoolOptions::new().connect(&uri).await?;
    Ok(result)
}
