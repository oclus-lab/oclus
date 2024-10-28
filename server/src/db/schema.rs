diesel::table! {
    users (id) {
        id -> BigSerial,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        refresh_token -> Nullable<Varchar>,
        registered_on -> Timestamp
    }
}

diesel::table! {
    groups (id) {
        id -> BigSerial,
        name -> Varchar,
        owner_id -> BigSerial,
        created_on -> Timestamp
    }
}

diesel::joinable!(groups -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(groups, users,);
