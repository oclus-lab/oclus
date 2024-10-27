use crate::dto::auth::TokenPair;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

const AUTH_TOKEN_LIFETIME: i64 = 60 * 60; // 1 hour
const REFRESH_TOKEN_LIFETIME: i64 = 60 * 60 * 24 * 30; // 1 month

pub enum TokenType {
    Auth,
    Refresh,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_token_pair(user_id: Uuid) -> TokenPair {
    TokenPair {
        auth_token: encode_token(user_id.clone(), &TokenType::Auth),
        refresh_token: encode_token(user_id.clone(), &TokenType::Refresh),
    }
}

fn encode_token(user_id: Uuid, token_type: &TokenType) -> String {
    let now = Utc::now().timestamp();

    let lifetime = match token_type {
        TokenType::Auth => AUTH_TOKEN_LIFETIME,
        TokenType::Refresh => REFRESH_TOKEN_LIFETIME,
    };

    let claims = Claims {
        sub: user_id,
        iat: now,
        exp: now + lifetime,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret_key(token_type)),
    )
    .expect("Failed to encode token")
}

pub fn decode_token(token: &str, token_type: &TokenType) -> Option<Claims> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&secret_key(token_type)),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims)
}

fn secret_key(token_type: &TokenType) -> Vec<u8> {
    let env_var_name = match token_type {
        TokenType::Auth => "AUTH_JWT_SECRET",
        TokenType::Refresh => "REFRESH_JWT_SECRET",
    };
    env::var(env_var_name)
        .expect(format!("{} must be set", env_var_name).as_str())
        .into_bytes()
}
