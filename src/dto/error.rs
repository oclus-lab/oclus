use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Serialize, Deserialize, Debug)]
pub enum ErrorDto {
    // ========= common errors =========
    #[error("Internal server error")]
    InternalServerError,

    #[error("Wrong data format")]
    InvalidData,

    // ========= auth errors =========
    #[error("Invalid token")]
    InvalidCredentials,

    // ========= user errors =========
    #[error("User not found in database")]
    NotFound,

    #[error("Email already exists in database")]
    Conflict(String),
}

impl ResponseError for ErrorDto {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
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

impl From<BlockingError> for ErrorDto {
    fn from(value: BlockingError) -> Self {
        log::error!("{}", value);
        Self::InternalServerError
    }
}
