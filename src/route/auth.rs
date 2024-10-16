use crate::{db, model};
use crate::dto::auth::{LoginRequest, RegisterRequest, TokenPair};
use crate::dto::error::ErrorDTO;
use crate::middleware::validation::ValidatedJson;
use crate::model::user;
use crate::model::user::*;
use crate::util::jwt::generate_token_pair;
use crate::util::sync::block_for_db;
use actix_web::{post, web};
use chrono::Utc;
use diesel::PgConnection;
use uuid::Uuid;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}

#[post("/auth/register")]
async fn register(
    request: ValidatedJson<RegisterRequest>,
    db_pool: web::Data<db::Pool>,
) -> Result<web::Json<TokenPair>, ErrorDTO> {
    let request = request.into_inner();
    let password_hash = bcrypt::hash(request.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

    let creation_data = CreateUser {
        email: request.email,
        username: request.username.clone(),
        password: password_hash,
        refresh_token: None,
        registration_date: Utc::now().naive_utc(),
    };

    let token_pair = block_for_db(&db_pool, move |mut db_conn| -> Result<TokenPair, ErrorDTO> {
        let user = create(creation_data, &mut db_conn).map_err(|error| match error {
            model::Error::UserEmailConflict => ErrorDTO::UserEmailConflict,
            _ => ErrorDTO::InternalServerError,
        })?;
        
        let token_pair = generate_token_pair(user.id);

        // save the refresh token in db
        save_refresh_token(&user.id, token_pair.refresh_token.clone(), &mut db_conn).map_err(|error| {
            if let model::Error::UserNotFound = error {
                log::error!("Data incoherence, created user {} doesn't exists anymore", user.id);
            }
            ErrorDTO::InternalServerError
        })?;

        Ok(token_pair)
    })
    .await??;

    Ok(web::Json(token_pair))
}

#[post("/auth/login")]
async fn login(
    request: ValidatedJson<LoginRequest>,
    db_pool: web::Data<db::Pool>,
) -> Result<web::Json<TokenPair>, ErrorDTO> {
    let request = request.into_inner();

    let token_pair = block_for_db(&db_pool, move |mut db_conn| -> Result<TokenPair, ErrorDTO> {
        let user = user::get_by_email(&request.email, &mut db_conn).map_err(|error| match error {
            model::Error::UserNotFound => ErrorDTO::InvalidCredentials,
            _ => ErrorDTO::InternalServerError,
        })?;

        bcrypt::verify(&request.password, &user.password).map_err(|_error| ErrorDTO::InvalidCredentials)?;

        let token_pair = generate_token_pair(user.id);

        // save the new refresh token in db
        save_refresh_token(&user.id, token_pair.refresh_token.clone(), &mut db_conn).map_err(|error| {
            if let model::Error::UserNotFound = error {
                log::error!(
                    "Data incoherence, previously found user {} doesn't exists anymore",
                    user.id
                );
            }
            ErrorDTO::InternalServerError
        })?;

        Ok(token_pair)
    })
    .await??;

    Ok(web::Json(token_pair))
}

fn save_refresh_token(user_id: &Uuid, refresh_token: String, db_conn: &mut PgConnection) -> Result<(), model::Error> {
    let update_data = UpdateUser::builder()
        .refresh_token(Some(Some(refresh_token)))
        .build()
        .unwrap();

    update(user_id, &update_data, db_conn)?;
    Ok(())
}
