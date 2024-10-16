use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Serialize, Debug)]
pub enum ErrorDTO {
    // ========= common errors =========
    #[error("Internal server error")]
    InternalServerError,

    #[error("Wrong data format")]
    WrongDataFormat,

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    // ========= auth errors =========
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    // ========= user errors =========
    #[error("User not found in database")]
    UserNotFound,

    #[error("Email already exists in database")]
    UserEmailConflict,
}

impl ResponseError for ErrorDTO {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WrongDataFormat => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials | Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::Validation(_error) => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::UserEmailConflict => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(Json(self))
    }
}

impl From<BlockingError> for ErrorDTO {
    fn from(value: BlockingError) -> Self {
        log::error!("{}", value);
        Self::InternalServerError
    }
}
