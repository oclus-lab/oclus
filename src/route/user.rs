use crate::db;
use crate::dto::error::ErrorDTO;
use crate::dto::user::{PrivateProfile, PublicProfile, UpdateProfileRequest};
use crate::middleware::auth::AuthInfo;
use crate::middleware::validation::ValidatedJson;
use crate::model::user::{delete_user, read_user, update_user, UpdateUser};
use actix_web::web::Json;
use actix_web::{delete, get, put, web};
use uuid::Uuid;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_private_profile);
    cfg.service(get_public_profile);
    cfg.service(update_profile);
    cfg.service(delete_profile);
}

#[get("/users/me")]
async fn get_private_profile(
    auth: AuthInfo,
    db_pool: web::Data<db::Pool>,
) -> Result<Json<PublicProfile>, ErrorDTO> {
    let user_id = auth.user_id;

    let user = web::block(move || {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");

        read_user(&user_id, &mut db_conn)
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

    let user = web::block(move || {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");

        read_user(&user_id, &mut db_conn)
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

    let user = web::block(move || {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");

        update_user(&user_id, &update_data, &mut db_conn)
    })
    .await??;

    Ok(Json(user.into()))
}

#[delete("/users/me/delete")]
async fn delete_profile(
    auth: AuthInfo,
    db_pool: web::Data<db::Pool>,
    password: String,
) -> Result<String, ErrorDTO> {
    let user_id = auth.user_id;

    web::block(move || -> Result<(), ErrorDTO> {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");

        let user = read_user(&auth.user_id, &mut db_conn)?;
        bcrypt::verify(password, &user.password).map_err(|_| ErrorDTO::InvalidCredentials)?;

        delete_user(&user_id, &mut db_conn)?;
        Ok(())
    })
    .await??;

    Ok("Account deleted successfully".to_string())
}
