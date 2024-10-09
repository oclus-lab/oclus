use crate::dto::auth::{LoginRequest, RegisterRequest, TokenPair};
use crate::dto::error::ErrorDTO;
use crate::middleware::validation::ValidatedJson;
use crate::model::user::{
    create_user, read_user, read_user_by_email, update_user, CreateUser, UpdateUser,
};
use crate::util::jwt::generate_token_pair;
use crate::{db, model};
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
    let password_hash =
        bcrypt::hash(request.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

    let creation_data = CreateUser {
        email: request.email,
        username: request.username.clone(),
        password: password_hash,
        refresh_token: None,
        registration_date: Utc::now().date_naive(),
    };

    let token_pair = web::block(move || -> Result<TokenPair, model::user::Error> {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");
        let user = create_user(creation_data, &mut db_conn)?;

        let token_pair = generate_token_pair(user.id);
        update_refresh_token(&user.id, token_pair.refresh_token.clone(), &mut db_conn)?;

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

    let token_pair = web::block(move || -> Result<TokenPair, ErrorDTO> {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");

        let user =
            read_user_by_email(&request.email, &mut db_conn).map_err(|error| match error {
                model::user::Error::UserNotFound => ErrorDTO::InvalidCredentials,
                _ => error.into(),
            })?;

        bcrypt::verify(&request.password, &user.password)
            .map_err(|_error| ErrorDTO::InvalidCredentials)?;

        let token_pair = generate_token_pair(user.id);
        update_refresh_token(&user.id, token_pair.refresh_token.clone(), &mut db_conn)?;
        Ok(token_pair)
    })
    .await??;

    Ok(web::Json(token_pair))
}

fn update_refresh_token(
    user_id: &Uuid,
    refresh_token: String,
    db_conn: &mut PgConnection,
) -> Result<(), model::user::Error> {
    let mut update_data = UpdateUser::default();
    update_data.refresh_token = Some(Some(refresh_token));

    update_user(user_id, &update_data, db_conn)?;
    Ok(())
}
