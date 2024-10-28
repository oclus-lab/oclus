use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use crate::db::model::group::GroupError;
use crate::db::model::user::UserError;

#[derive(thiserror::Error, Serialize, Deserialize, Debug)]
pub enum ErrorDto {
    // ========= common errors =========
    
    #[error("internal server error")]
    Internal,

    #[error("wrong data format")]
    InvalidData,

    // ========= auth errors =========
    
    #[error("invalid credentials")]
    InvalidCredentials,

    // ========= user errors =========
    
    #[error("not found")]
    NotFound,

    #[error("field {0} already exists")]
    Conflict(String),
}

impl ResponseError for ErrorDto {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidData => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_field) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(Json(self))
    }
}

// ========= common default conversions to ErrorDTO =========

impl From<BlockingError> for ErrorDto {
    fn from(_value: BlockingError) -> Self {
        Self::Internal
    }
}

impl From<UserError> for ErrorDto {
    fn from(_value: UserError) -> Self {
        Self::Internal
    }
}

impl From<GroupError> for ErrorDto {
    fn from(_value: GroupError) -> Self {
        Self::Internal
    }
}
