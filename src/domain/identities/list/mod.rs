mod controller;
pub use controller::controller;
use sqlx::PgPool;    
use crate::io::result::Error;
use super::types::CreatedEvent;

pub async fn handler(db: &PgPool) -> Result<Vec<CreatedEvent>, Error> {
    let result = sqlx::query_as::<_, CreatedEvent>("select * from identities.events")
        .fetch_all(db)
        .await?;

    // We only support a single CREATED event, so no reduction is needed.
    Ok(result)
}
