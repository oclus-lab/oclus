use crate::db::{DbConnection, DbPool};
use crate::dto::auth::{LoginRequest, RegisterRequest, TokenPair};
use crate::dto::error::ErrorDto;
use crate::middleware::validation::ValidatedJson;
use crate::model;
use crate::model::user;
use crate::model::user::{UserCreationData, UserUpdateData};
use crate::util::crypto::{hash_password, verify_password};
use crate::util::db::{block_for_db, block_for_trans_db};
use crate::util::jwt::generate_token_pair;
use actix_web::{post, web};
use actix_web::web::Json;
use chrono::Utc;
use uuid::Uuid;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}

#[post("/auth/register")]
async fn register(
    request: ValidatedJson<RegisterRequest>,
    db_pool: web::Data<DbPool>,
) -> Result<Json<TokenPair>, ErrorDto> {
    let request = request.into_inner();

    let creation_data = UserCreationData {
        email: request.email,
        username: request.username.clone(),
        password: hash_password(&request.password),
        refresh_token: None,
        registration_date: Utc::now().naive_utc(),
    };

    let token_pair = block_for_trans_db(&db_pool, move |mut conn| {
        let user = user::create(creation_data, &mut conn)?;

        let token_pair = generate_token_pair(user.id);

        // save the refresh token in db
        save_refresh_token(&user.id, token_pair.refresh_token.clone(), &mut conn)?;

        Ok(token_pair)
    })
    .await?
    .map_err(|error| match error {
        model::Error::Conflict(field) => ErrorDto::Conflict(field),
        _ => ErrorDto::InternalServerError,
    })?;

    Ok(Json(token_pair))
}

#[post("/auth/login")]
async fn login(
    request: ValidatedJson<LoginRequest>,
    db_pool: web::Data<DbPool>,
) -> Result<Json<TokenPair>, ErrorDto> {
    let request = request.into_inner();

    let user = block_for_db(&db_pool, move |mut conn| {
        user::get_by_email(&request.email, &mut conn)
    })
    .await?
    .map_err(|err| match err {
        model::Error::NotFound => ErrorDto::InvalidCredentials,
        _ => ErrorDto::InternalServerError,
    })?;

    if !verify_password(&request.password, &user.password) {
        return Err(ErrorDto::InvalidCredentials);
    }

    let token_pair = generate_token_pair(user.id);

    let refresh_token = token_pair.refresh_token.clone();
    block_for_db(&db_pool, move |mut conn| {
        save_refresh_token(&user.id, refresh_token, &mut conn)
    })
    .await?
    .map_err(|err| {
        log::error!("Error saving refresh token {:?}", err);
        ErrorDto::InternalServerError
    })?;

    Ok(Json(token_pair))
}

fn save_refresh_token(
    user_id: &Uuid,
    refresh_token: String,
    db_conn: &mut DbConnection,
) -> Result<(), model::Error> {
    let update_data = UserUpdateData::builder()
        .refresh_token(Some(Some(refresh_token)))
        .build()
        .unwrap();

    user::update(user_id, &update_data, db_conn)?;
    Ok(())
}
