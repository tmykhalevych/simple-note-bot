// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        user_id -> Int4,
        url -> Text,
        timestamp -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        external_id -> Text,
    }
}

diesel::joinable!(notes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
