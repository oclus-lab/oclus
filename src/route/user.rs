use crate::db;
use crate::dto::error::ErrorDTO;
use crate::dto::user::{PrivateProfile, PublicProfile, UpdateProfileRequest};
use crate::middleware::auth::AuthInfo;
use crate::middleware::validation::ValidatedJson;
use crate::model::user;
use crate::model::user::{delete_user, read_user, update_user, UpdateUser};
use crate::util::sync::block_for_db;
use actix_web::web::Json;
use actix_web::{delete, get, put, web, HttpResponse};
use uuid::Uuid;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_private_profile);
    cfg.service(get_public_profile);
    cfg.service(update_profile);
    cfg.service(delete_profile);
}

#[get("/users/me")]
async fn get_private_profile(auth: AuthInfo, db_pool: web::Data<db::Pool>) -> Result<Json<PublicProfile>, ErrorDTO> {
    let user_id = auth.user_id;

    let user = block_for_db(&db_pool, move |mut db_conn| {
        read_user(&user_id, &mut db_conn).map_err(|error| match error {
            user::Error::UserNotFound => ErrorDTO::UserNotFound,
            _ => ErrorDTO::InternalServerError,
        })
    })
    .await??;

    Ok(Json(user.into()))
}

#[get("/users/{user_id}")]
async fn get_public_profile(
    _auth: AuthInfo,
    db_pool: web::Data<db::Pool>,
    user_id: web::Path<Uuid>,
) -> Result<Json<PublicProfile>, ErrorDTO> {
    let user_id = user_id.into_inner();

    let user = block_for_db(&db_pool, move |mut db_conn| {
        read_user(&user_id, &mut db_conn).map_err(|error| match error {
            user::Error::UserNotFound => ErrorDTO::UserNotFound,
            _ => ErrorDTO::InternalServerError,
        })
    })
    .await??;

    Ok(Json(user.into()))
}

#[put("/users/me/update")]
async fn update_profile(
    auth: AuthInfo,
    db_pool: web::Data<db::Pool>,
    request: ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<PrivateProfile>, ErrorDTO> {
    let user_id = auth.user_id;
    let request = request.into_inner();

    let update_data = UpdateUser {
        email: request.email,
        username: request.username,
        password: None,
        refresh_token: None,
        registration_date: None,
    };

    let user = block_for_db(&db_pool, move |mut db_conn| {
        update_user(&user_id, &update_data, &mut db_conn)
    })
    .await?
    .map_err(|error| {
        if let user::Error::UserNotFound = error {
            log::warn!("Authenticated user not found in database");
        }
        ErrorDTO::InternalServerError
    })?;

    Ok(Json(user.into()))
}

#[delete("/users/me/delete")]
async fn delete_profile(
    auth: AuthInfo,
    db_pool: web::Data<db::Pool>,
    password: String, // authenticated user have to provide his password
) -> Result<HttpResponse, ErrorDTO> {
    block_for_db(&db_pool, move |mut db_conn| -> Result<(), ErrorDTO> {
        let user = read_user(&auth.user_id, &mut db_conn).map_err(|error| match error {
            user::Error::UserNotFound => ErrorDTO::InvalidCredentials,
            _ => ErrorDTO::InternalServerError,
        })?;

        // password verification
        bcrypt::verify(password, &user.password).map_err(|_| ErrorDTO::InvalidCredentials)?;

        delete_user(&auth.user_id, &mut db_conn).map_err(|error|  {
            if let user::Error::UserNotFound = error {
                log::error!("Data incoherence, user {} found doesn't exists", user.id);
            }

            ErrorDTO::InternalServerError
        })?;
        Ok(())
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}
