use crate::app::service::{ServiceError, ServiceResult};
use crate::model::user::User;
use crate::util::crypto::{gen_totp, hash};
use actix_web::web::Json;
use actix_web::{post, web};
use chrono::{NaiveDateTime, TimeDelta, Utc};
use serde::Deserialize;
use sqlx::PgPool;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(register_confirm);
}

#[post("/auth/register")]
async fn register(db_pool: web::Data<PgPool>, email: Json<String>) -> ServiceResult<Json<i64>> {
    let email = email.into_inner();

    let email_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
        "#,
    )
    .bind(email.clone())
    .fetch_one(db_pool.as_ref())
    .await?;

    if email_exists {
        return Err(ServiceError::Conflict("email".to_string()).into());
    }

    let registration_req_id = sqlx::query_scalar(
        r#"
        INSERT INTO registration_requests (email, totp, sent_on)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(gen_totp())
    .bind(Utc::now())
    .fetch_one(db_pool.get_ref())
    .await?;

    // TODO: send verification email

    Ok(Json(registration_req_id))
}

#[derive(Deserialize, Debug)]
pub struct RegistrationData {
    pub req_id: i64, // registration request id
    pub totp: String,
    pub username: String,
    pub password: String,
}

#[post("/auth/register/confirm")]
async fn register_confirm(
    db_pool: web::Data<PgPool>,
    data: Json<RegistrationData>,
) -> ServiceResult<Json<User>> {
    let data = data.into_inner();

    let (email, totp, sent_on, trials): (String, String, NaiveDateTime, i32) = sqlx::query_as(
        r#"
        SELECT email, totp, sent_on, trials FROM registration_requests
        WHERE id = $1
        "#,
    )
    .bind(data.req_id.clone())
    .fetch_optional(db_pool.as_ref())
    .await?
    .ok_or(ServiceError::NotFound)?;

    // check the request validity and the totp
    const REQ_LIFETIME: TimeDelta = TimeDelta::minutes(15); // 15 minutes
    if (Utc::now() - sent_on.and_utc()) > REQ_LIFETIME {
        log::info!("expired registration request {} usage attempt", data.req_id);
        return Err(ServiceError::Unauthorized);
    }

    // check the totp
    const MAX_TRIALS: i32 = 3;
    if trials >= MAX_TRIALS || totp != data.totp {
        log::info!("wrong totp used for registration request {}", data.req_id);

        // increment trials counter
        sqlx::query(
            r#"
            UPDATE registration_requests
            SET trials = trials + 1
            WHERE id = $1
            "#,
        )
        .bind(data.req_id)
        .execute(db_pool.as_ref())
        .await?;

        return Err(ServiceError::Unauthorized);
    }

    // start transaction
    let mut txn = db_pool.begin().await?;

    // create user
    let user: User = sqlx::query_as(
        r#"
        INSERT INTO users (email, username, registered_on)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, registered_on
        "#,
    )
    .bind(email.clone()) // email
    .bind(data.username) // username
    .bind(Utc::now().naive_utc()) // registered_on
    .fetch_one(&mut *txn)
    .await?;

    // create user auth infos
    sqlx::query(
        r#"
        INSERT INTO auth_infos (user_id, user_email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(email.clone())
    .bind(hash(&data.password))
    .execute(&mut *txn)
    .await?;

    // delete all registration requests for the verified email
    sqlx::query(
        r#"
        DELETE FROM registration_requests WHERE email = $1
        "#,
    )
    .bind(email.clone())
    .execute(&mut *txn)
    .await?;

    // commit transaction
    txn.commit().await?;

    Ok(Json(user))
}
