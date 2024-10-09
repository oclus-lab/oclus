use crate::model;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::BlockingError;
use serde::Serialize;

#[derive(thiserror::Error, Serialize, Debug)]
pub enum ErrorDTO {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Wrong data format")]
    WrongDataFormat,

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error("User not found in database")]
    UserNotFound,
}

impl ResponseError for ErrorDTO {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WrongDataFormat => StatusCode::BAD_REQUEST,
            Self::Validation(_error) => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body: String = match self {
            Self::Validation(error) => serde_json::to_string(error).unwrap_or_default(),
            _ => self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<BlockingError> for ErrorDTO {
    fn from(value: BlockingError) -> Self {
        log::error!("{}", value);
        Self::InternalServerError
    }
}

impl From<model::user::Error> for ErrorDTO {
    fn from(value: model::user::Error) -> Self {
        match value {
            model::user::Error::UserNotFound => ErrorDTO::UserNotFound,
            _ => {
                log::error!("{}", value);
                ErrorDTO::InternalServerError
            },
        }
    }
}