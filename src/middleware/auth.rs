use crate::dto::error::ErrorDTO;
use crate::util::jwt::{decode_token, TokenType};
use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

pub struct AuthInfo {
    pub user_id: Uuid,
}

impl FromRequest for AuthInfo {
    type Error = ErrorDTO;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION).cloned();

        Box::pin(async move {
            let auth_header_value = auth_header.ok_or(ErrorDTO::InvalidToken)?;
            let auth_header_str = auth_header_value
                .to_str()
                .map_err(|_| ErrorDTO::InvalidToken)?;

            let token = auth_header_str
                .strip_prefix("Bearer ")
                .ok_or(ErrorDTO::InvalidToken)?;

            let claims = decode_token(token, &TokenType::Auth).ok_or(ErrorDTO::InvalidToken)?;
            Ok(AuthInfo {
                user_id: claims.sub,
            })
        })
    }
}
