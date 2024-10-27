use crate::dto::error::ErrorDto;
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Validate + DeserializeOwned + 'static> FromRequest for ValidatedJson<T> {
    type Error = ErrorDto;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_future = web::Json::<T>::from_request(req, payload);
        Box::pin(async move {
            match json_future.await {
                // data validation using validator
                Ok(data) if data.validate().is_ok() => Ok(ValidatedJson(data.into_inner())),
                _ => Err(ErrorDto::InvalidData),
            }
        })
    }
}
