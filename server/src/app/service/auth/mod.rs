use crate::app::service::ServiceError;
use crate::util::crypto::{decode_jwt, encode_jwt, verify_hash};
use actix_web::web::Json;
use actix_web::{post, web};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod register;

const AUTH_TOKEN_LIFETIME: Duration = Duration::minutes(10);
const REFRESH_TOKEN_LIFETIME: Duration = Duration::days(28);

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(refresh);
    register::configure(cfg);
}

#[derive(Serialize, Debug)]
pub struct TokenPair {
    pub auth: String,
    pub refresh: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[post("/auth/login")]
async fn login(
    db_pool: web::Data<PgPool>,
    data: Json<LoginData>,
) -> Result<Json<TokenPair>, ServiceError> {
    let data = data.into_inner();
    let (user_id, password_hash): (i64, String) = sqlx::query_as(
        r#"
        SELECT user_id, password_hash FROM auth_infos WHERE user_email = $1
        "#,
    )
    .bind(data.email.clone())
    .fetch_optional(db_pool.as_ref())
    .await?
    .ok_or(ServiceError::Unauthorized)?;

    if !verify_hash(&data.password, &password_hash) {
        return Err(ServiceError::Unauthorized);
    }

    let auth_token = encode_jwt(user_id, AUTH_TOKEN_LIFETIME);
    let refresh_token = encode_jwt(user_id, REFRESH_TOKEN_LIFETIME);

    // save the new refresh token in db
    sqlx::query(
        r#"
        UPDATE auth_infos SET refresh_token = $1 WHERE user_id = $2
        "#,
    )
    .bind(refresh_token.clone())
    .bind(user_id)
    .execute(db_pool.as_ref())
    .await?;

    let token_pair = TokenPair {
        auth: auth_token,
        refresh: refresh_token,
    };

    Ok(Json(token_pair))
}

#[post("/auth/refresh")]
async fn refresh(
    db_pool: web::Data<PgPool>,
    refresh_token: String,
) -> Result<Json<TokenPair>, ServiceError> {
    let user_id = decode_jwt(&refresh_token).ok_or_else(|| {
        log::info!("invalid refresh token provided");
        ServiceError::Unauthorized
    })?;

    let stored_token: String = sqlx::query_scalar(
        r#"
        SELECT refresh_token FROM auth_infos WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db_pool.as_ref())
    .await?
    .ok_or_else(|| {
        log::warn!("valid refresh token provided but user not found");
        ServiceError::Unauthorized
    })?;

    match stored_token == refresh_token {
        false => Err(ServiceError::Unauthorized),
        true => {
            let new_auth_token = encode_jwt(user_id, AUTH_TOKEN_LIFETIME);
            let new_refresh_token = encode_jwt(user_id, REFRESH_TOKEN_LIFETIME);

            // store the new refresh token in db
            sqlx::query(r#"UPDATE auth_infos SET refresh_token = $1 WHERE user_id = $2"#)
                .bind(new_refresh_token.clone())
                .bind(user_id)
                .execute(db_pool.as_ref())
                .await?;

            log::info!("auth token refreshed for user {}", user_id);

            let token_pair = TokenPair {
                auth: new_auth_token,
                refresh: new_refresh_token,
            };

            Ok(Json(token_pair))
        }
    }
}
