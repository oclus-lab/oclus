use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use totp_rs::TOTP;

pub fn hash(data: &str) -> String {
    bcrypt::hash(data, bcrypt::DEFAULT_COST).expect("failed to hash")
}

pub fn verify_hash(data: &str, hash: &str) -> bool {
    bcrypt::verify(data, hash).unwrap_or(false)
}

/// generate a one time password
pub fn gen_totp() -> String {
    let secret_var = env::var("TOTP_SECRET").expect("TOTP_SECRET is not set");
    let secret = totp_rs::Secret::Raw(secret_var.into_bytes())
        .to_bytes()
        .map_err(|err| {
            log::error!("failed to encode secret: {}", err);
            err
        })
        .unwrap();

    let totp =
        TOTP::new(totp_rs::Algorithm::SHA512, 6, 1, 30, secret).expect("failed to init TOTP");
    totp.generate_current().expect("failed to generate totp")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

/// Encode a token for a user id
pub fn encode_jwt(user_id: i64, lifetime: Duration) -> String {
    let exp = Utc::now() + lifetime;
    let claims = Claims {
        sub: user_id,
        exp: exp.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);

    let key_var = env::var("JWT_SECRET").expect("JWT_SECRET is not set");
    let key = jsonwebtoken::EncodingKey::from_secret(key_var.as_bytes());

    jsonwebtoken::encode(&header, &claims, &key).expect("failed to encode jwt")
}

/// Decode a token and return the user id
/// Return None if the token is invalid or expired
pub fn decode_jwt(token: &str) -> Option<i64> {
    let key_var = env::var("JWT_SECRET").expect("JWT_SECRET is not set");
    let key = jsonwebtoken::DecodingKey::from_secret(key_var.as_bytes());

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512);
    jsonwebtoken::decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims.sub)
        .ok()
}
