// @generated automatically by Diesel CLI.

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
