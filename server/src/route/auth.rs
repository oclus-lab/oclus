use crate::db::model::user::{User, UserUpdateData};
use crate::db::DbPool;
use crate::dto::auth::{LoginRequest, TokenPair};
use crate::dto::error::ErrorDto;
use crate::util::crypto::verify_password;
use crate::util::db::block_for_db;
use crate::util::jwt::{decode_token, generate_token_pair, TokenType};
use actix_web::web::Json;
use actix_web::{post, web};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(refresh_auth);
}

#[post("/auth/login")]
async fn login(
    request: Json<LoginRequest>,
    db_pool: web::Data<DbPool>,
) -> Result<Json<TokenPair>, ErrorDto> {
    let request = request.into_inner();

    let user = block_for_db(&db_pool, move |conn| {
        User::get_by_email(&request.email, conn)
    })
    .await??
    .ok_or(ErrorDto::InvalidCredentials)?; // user not found

    if !verify_password(&request.password, &user.password) {
        return Err(ErrorDto::InvalidCredentials);
    }

    let token_pair = generate_token_pair(user.id);

    // save the new refresh token in db
    let refresh_token = token_pair.refresh_token.clone();
    block_for_db(&db_pool, move |conn| {
        let mut user_update = UserUpdateData::default();
        user_update.refresh_token = Some(Some(refresh_token));
        User::update(user.id, &user_update, conn)
    })
    .await??;

    Ok(Json(token_pair))
}

#[post("/auth/refresh")]
async fn refresh_auth(
    refresh_token: String,
    db_pool: web::Data<DbPool>,
) -> Result<Json<TokenPair>, ErrorDto> {
    let claims =
        decode_token(&refresh_token, &TokenType::Refresh).ok_or(ErrorDto::InvalidCredentials)?;

    let user = block_for_db(&db_pool, move |conn| User::get(claims.sub, conn))
        .await??
        .ok_or_else(|| {
            log::warn!(
                "valid refresh token provided, but subject user {} not found",
                claims.sub
            );
            ErrorDto::InvalidCredentials
        })?;

    if user.refresh_token == Some(refresh_token) {
        let token_pair = generate_token_pair(user.id);

        // save the new refresh token in db
        let refresh_token = token_pair.refresh_token.clone();
        block_for_db(&db_pool, move |conn| {
            let mut user_update = UserUpdateData::default();
            user_update.refresh_token = Some(Some(refresh_token));
            User::update(user.id, &user_update, conn)
        })
        .await??;

        Ok(Json(token_pair))
    } else {
        Err(ErrorDto::InvalidCredentials)
    }
}
