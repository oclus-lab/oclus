use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

const USERNAME_MIN_LEN: u64 = 4;
const USERNAME_MAX_LEN: u64 = 32;
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9A-Za-z-_]").unwrap());
const PASSWORD_MIN_LEN: u64 = 16;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = USERNAME_MIN_LEN, max = USERNAME_MAX_LEN), regex(path = *USERNAME_REGEX))]
    pub username: String,

    #[validate(length(min = PASSWORD_MIN_LEN))]
    pub password: String,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct LoginRequest {
    #[validate(length(min = USERNAME_MIN_LEN))]
    pub identifier: String,

    #[validate(length(min = PASSWORD_MIN_LEN))]
    pub password: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenPair {
    pub auth_token: String,
    pub refresh_token: String,
}
