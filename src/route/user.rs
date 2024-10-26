use crate::db::DbPool;
use crate::dto::error::ErrorDto;
use crate::dto::user::{PrivateProfile, PublicProfile, UpdateProfileRequest};
use crate::middleware::auth::strong::StrongAuthGuard;
use crate::middleware::auth::AuthGuard;
use crate::middleware::validation::ValidatedJson;
use crate::model;
use crate::model::user;
use crate::model::user::{UserUpdateData};
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
        user::get(&user_id, &mut db_conn)
    })
    .await?
    .map_err(|error| match error {
        model::Error::NotFound => ErrorDto::NotFound,
        _ => ErrorDto::InternalServerError,
    })?;

    Ok(Json(user.into()))
}

#[get("/users/{user_id}")]
async fn get_public_profile(
    _auth: AuthGuard,
    db_pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<Json<PublicProfile>, ErrorDto> {
    let user_id = user_id.into_inner();

    let user = block_for_db(&db_pool, move |mut db_conn| {
        user::get(&user_id, &mut db_conn)
    })
    .await?
    .map_err(|error| match error {
        model::Error::NotFound => ErrorDto::NotFound,
        _ => ErrorDto::InternalServerError,
    })?;

    Ok(Json(user.into()))
}

#[put("/users/me/update")]
async fn update_profile(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
    request: ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<PrivateProfile>, ErrorDto> {
    let user_id = auth.user_id;
    let request = request.into_inner();

    let update_data = UserUpdateData {
        email: request.email,
        username: request.username,
        password: None,
        refresh_token: None,
        registration_date: None,
    };

    let user = block_for_db(&db_pool, move |mut db_conn| {
        user::update(&user_id, &update_data, &mut db_conn)
    })
    .await?
    .map_err(|error| {
        if let model::Error::NotFound = error {
            log::warn!("Authenticated user {} not found", auth.user_id);
        }
        ErrorDto::InternalServerError
    })?;

    Ok(Json(user.into()))
}

#[delete("/users/me/delete")]
async fn delete_user(
    auth: StrongAuthGuard,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, ErrorDto> {
    block_for_db(&db_pool, move |mut db_conn| {
        user::delete(&auth.user_id, &mut db_conn)
    })
    .await?
    .map_err(|error| {
        if let model::Error::NotFound = error {
            log::warn!("Authenticated user {} not found", auth.user_id);
        }
        ErrorDto::InternalServerError
    })?;

    Ok(HttpResponse::Ok().finish())
}
