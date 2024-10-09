use crate::db;
use crate::dto::auth::{LoginRequest, RegisterRequest, TokenPair};
use crate::dto::error::ErrorDTO;
use crate::middleware::validation::ValidatedJson;
use crate::model::user::{create_user, CreateUser};
use actix_web::{post, web};
use chrono::Utc;
use crate::util::jwt::generate_token_pair;

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
        display_name: request.username,
        registration_date: Utc::now().date_naive(),
    };

    let user = web::block(move || {
        let mut db_conn = db_pool
            .get()
            .expect("Failed to get db connection from pool");
        create_user(creation_data, &mut db_conn)
    }).await??;

    let token_pair = generate_token_pair(user.id);
    Ok(web::Json(token_pair))
}

#[post("/auth/login")]
async fn login(
    request: ValidatedJson<LoginRequest>,
    db_pool: web::Data<db::Pool>,
) -> Result<web::Json<TokenPair>, ErrorDTO> {
    todo!()
}
