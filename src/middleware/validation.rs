use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::{future::Future, pin::Pin};
use validator::Validate;

use crate::dto::error::ErrorDTO;

pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Validate + DeserializeOwned + 'static> FromRequest for ValidatedJson<T> {
    type Error = ErrorDTO;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // async json extractor usage
        let json_future = web::Json::<T>::from_request(req, payload);
        Box::pin(async move {
            match json_future.await {
                // data validation with validator
                Ok(data) => match data.validate() {
                    Ok(_) => Ok(ValidatedJson(data.into_inner())),
                    Err(error) => Err(ErrorDTO::Validation(error)),
                },
                Err(_error) => Err(ErrorDTO::WrongDataFormat),
            }
        })
    }
}
