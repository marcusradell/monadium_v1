use super::schema::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable, Insertable)]
#[table_name = "invitation"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
