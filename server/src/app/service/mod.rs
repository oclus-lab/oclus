use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub mod auth;
pub mod dm;
pub mod group;
pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
    user::configure(cfg);
    auth::configure(cfg);
}

#[derive(Error, Serialize, Debug)]
pub enum ServiceError {
    #[error("internal server error")]
    InternalServer,

    #[error("unauthorized")]
    Unauthorized,
    
    #[error("not found")]
    NotFound,

    #[error("conflict for field {0}")]
    Conflict(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::Conflict(_field) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(Json(self))
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("{}", err);
        ServiceError::InternalServer
    }
}
