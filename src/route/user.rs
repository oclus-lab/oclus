use crate::db::model::user::{User, UserUpdateData};
use crate::db::model::DbError;
use crate::db::DbPool;
use crate::dto::error::ErrorDto;
use crate::dto::user::{PrivateProfile, PublicProfile, UpdateProfileRequest};
use crate::middleware::auth::strong::StrongAuthGuard;
use crate::middleware::auth::AuthGuard;
use crate::middleware::validation::ValidatedJson;
use crate::util::db::block_for_db;
use actix_web::web::Json;
use actix_web::{delete, get, put, web, HttpResponse};
use uuid::Uuid;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_private_profile);
    cfg.service(get_public_profile);
    cfg.service(update_profile);
    cfg.service(delete_user);
}

#[get("/users/me")]
async fn get_private_profile(
    auth: AuthGuard,
    db_pool: web::Data<DbPool>,
) -> Result<Json<PublicProfile>, ErrorDto> {
    let user_id = auth.user_id;

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::get(&user_id, &mut db_conn)
    })
    .await?
    .map_err(|_err| ErrorDto::InternalServerError)?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(ErrorDto::NotFound),
    }
}

#[get("/users/{user_id}")]
async fn get_public_profile(
    _auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<Json<PublicProfile>, ErrorDto> {
    let user_id = user_id.into_inner();

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::get(&user_id, &mut db_conn)
    })
    .await?
    .map_err(|_err| ErrorDto::InternalServerError)?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(ErrorDto::NotFound),
    }
}

#[put("/users/me/update")]
async fn update_profile(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
    request: ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<PrivateProfile>, ErrorDto> {
    let user_id = auth.user_id;
    let request = request.into_inner();

    let mut user_update = UserUpdateData::default();
    user_update.email = request.email;
    user_update.username = request.username;

    let user = block_for_db(&db_pool, move |mut db_conn| {
        User::update(&user_id, &user_update, &mut db_conn)
    })
    .await?
    .map_err(|err| match err {
        DbError::NotFound => {
            log::warn!("Authenticated user {} not found", auth.user_id);
            ErrorDto::NotFound
        }
        DbError::Conflict(field) => ErrorDto::Conflict(field),
        _ => {
            log::error!("Error while updating user: {}", err);
            ErrorDto::InternalServerError
        }
    })?;

    Ok(Json(user.into()))
}

#[delete("/users/me/delete")]
async fn delete_user(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ErrorDto> {
    block_for_db(&db_pool, move |mut db_conn| {
        User::delete(&auth.user_id, &mut db_conn)
    })
    .await?
    .map_err(|error| {
        if let DbError::NotFound = error {
            log::warn!("Authenticated user {} not found", auth.user_id);
        }
        ErrorDto::InternalServerError
    })?;

    Ok(HttpResponse::Ok().finish())
}
