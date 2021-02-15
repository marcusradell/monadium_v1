use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Identity {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

use crate::schema::identity;

#[derive(Insertable, Debug)]
#[table_name = "identity"]
pub struct NewIdentity<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}
