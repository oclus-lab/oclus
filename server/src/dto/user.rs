use std::sync::OnceLock;
use crate::db::model::user::User;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

const USERNAME_MIN_LEN: u64 = 4;
const USERNAME_MAX_LEN: u64 = 32;
const PASSWORD_MIN_LEN: u64 = 12;

fn username_regex() -> &'static Regex {
    static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();
    USERNAME_REGEX.get_or_init(|| Regex::new(r"[0-9A-Za-z-_]").unwrap())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub registered_on: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPublicDto {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = USERNAME_MIN_LEN, max = USERNAME_MAX_LEN), regex(path = username_regex()))]
    pub username: String,

    #[validate(length(min = PASSWORD_MIN_LEN))]
    pub password: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = USERNAME_MIN_LEN, max = USERNAME_MAX_LEN), regex(path = username_regex()))]
    pub username: Option<String>,
}

impl From<User> for UserPublicDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            registered_on: value.registered_on.and_utc(),
        }
    }
}
