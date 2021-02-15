// identity is placed in the identity app. But what do we do with allow_tables_to_appear?

// table! {
//     identity (id) {
//         id -> Int4,
//         email -> Varchar,
//         password_hash -> Varchar,
//         created_at -> Timestamp,
//     }
// }

table! {
    invitation (id) {
        id -> Uuid,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(identity, invitation,);
