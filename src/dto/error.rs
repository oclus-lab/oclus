use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Serialize, Debug)]
pub enum ErrorDetail {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Wrong data format")]
    WrongDataFormat,

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

impl ResponseError for ErrorDetail {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WrongDataFormat => StatusCode::BAD_REQUEST,
            Self::Validation(_error) => StatusCode::BAD_REQUEST,
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
