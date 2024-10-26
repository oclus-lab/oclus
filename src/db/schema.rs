// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Varchar,
        owner_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        refresh_token -> Nullable<Varchar>,
        registration_date -> Timestamp,
    }
}

diesel::joinable!(groups -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(groups, users,);
