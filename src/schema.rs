// @generated automatically by Diesel CLI.

diesel::table! {
    auth (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 6]
        pin -> Varchar,
        tries -> Int4,
        expires_at -> Timestamptz,
        locked_until -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    boards (id) {
        id -> Int4,
        name -> Text,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        email_verified_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    widgets (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(auth -> users (user_id));
diesel::joinable!(boards -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    boards,
    users,
    widgets,
);
