#[derive(Queryable)]
pub struct Identity {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

use super::schema::identity;

#[derive(Insertable)]
#[table_name = "identity"]
pub struct NewIdentity<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}
