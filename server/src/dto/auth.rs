use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use validator::Validate;

const USERNAME_MIN_LEN: u64 = 4;
const USERNAME_MAX_LEN: u64 = 32;
const PASSWORD_MIN_LEN: u64 = 12;

fn username_regex() -> &'static Regex {
    static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();
    USERNAME_REGEX.get_or_init(|| Regex::new(r"[0-9A-Za-z-_]").unwrap())
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = USERNAME_MIN_LEN, max = USERNAME_MAX_LEN), regex(path = username_regex()))]
    pub username: String,

    #[validate(length(min = PASSWORD_MIN_LEN))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = PASSWORD_MIN_LEN))]
    pub password: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenPair {
    pub auth_token: String,
    pub refresh_token: String,
}
