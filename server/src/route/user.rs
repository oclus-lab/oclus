use crate::db::model::user::{User, UserCreationData, UserError, UserUpdateData};
use crate::db::DbPool;
use crate::dto::error::ErrorDto;
use crate::dto::user::{CreateUserRequest, UpdateUserRequest, UserDto, UserPublicDto};
use crate::middleware::auth::strong::StrongAuthGuard;
use crate::middleware::auth::AuthGuard;
use crate::middleware::validation::ValidatedJson;
use crate::util::crypto::hash_password;
use crate::util::db::block_for_db;
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_user);
    cfg.service(get_user_public);
}

#[post("/users/create")]
async fn create_user(
    db_pool: web::Data<DbPool>,
    request: ValidatedJson<CreateUserRequest>,
) -> Result<Json<UserDto>, ErrorDto> {
    let request = request.into_inner();

    let creation_data = UserCreationData {
        email: request.email,
        username: request.username.clone(),
        password: hash_password(&request.password),
        refresh_token: None,
        registered_on: Utc::now().naive_utc(),
    };

    let user = block_for_db(&db_pool, move |mut conn| {
        User::create(&creation_data, &mut conn)
    })
    .await?
    .map_err(|error| match error {
        UserError::EmailConflict(_email) => ErrorDto::Conflict(String::from("email")),
        _ => ErrorDto::Internal,
    })?;

    Ok(Json(user.into()))
}

#[put("/users/me/update")]
async fn update_user(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
    request: ValidatedJson<UpdateUserRequest>,
) -> Result<Json<UserDto>, ErrorDto> {
    let user_id = auth.user_id;
    let request = request.into_inner();

    let mut user_update = UserUpdateData::default();
    user_update.email = request.email;
    user_update.username = request.username;

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::update(user_id, &user_update, &mut db_conn)
    })
    .await?
    .map_err(|err| match err {
        UserError::EmailConflict(email) => ErrorDto::Conflict(email),
        UserError::UserNotFound(id) => {
            log::warn!("authenticated user {} not found", id);
            ErrorDto::NotFound
        }
        _ => ErrorDto::Internal,
    })?;

    Ok(Json(user.into()))
}

#[delete("/users/me/delete")]
async fn delete_user(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ErrorDto> {
    block_for_db(&db_pool, move |mut db_conn| {
        User::delete(auth.user_id, &mut db_conn)
    })
    .await?
    .map_err(|err| match err {
        UserError::UserNotFound(id) => {
            log::warn!("authenticated user {} not found", id);
            ErrorDto::NotFound
        }
        _ => ErrorDto::Internal,
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/users/me")]
async fn get_user(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
) -> Result<Json<UserPublicDto>, ErrorDto> {
    let user_id = auth.user_id;

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::get(user_id, &mut db_conn)
    })
    .await??;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(ErrorDto::NotFound),
    }
}

#[get("/users/{user_id}")]
async fn get_user_public(
    _auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    user_id: web::Path<i64>,
) -> Result<Json<UserPublicDto>, ErrorDto> {
    let user_id = user_id.into_inner();

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::get(user_id, &mut db_conn)
    })
    .await??;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(ErrorDto::NotFound),
    }
}
