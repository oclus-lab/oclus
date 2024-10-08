use crate::dto::error::ErrorDetail;
use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;
use validator::Validate;

pub struct Validated<T>(pub T);

impl<T: Validate + DeserializeOwned + 'static> FromRequest for Validated<T> {
    type Error = ErrorDetail;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_future = Json::<T>::from_request(req, payload);

        Box::pin(async move {
            match json_future.await {
                Ok(data) => match data.validate() {
                    Ok(_) => Ok(Validated(data.into_inner())),
                    Err(error) => Err(ErrorDetail::Validation(error)),
                },
                Err(_error) => Err(ErrorDetail::WrongDataFormat),
            }
        })
    }
}
