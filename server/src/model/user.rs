use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub registered_on: NaiveDateTime,
}

#[derive(Serialize, FromRow, Debug)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
}
