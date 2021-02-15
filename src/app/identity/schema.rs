table! {
    identity (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}
