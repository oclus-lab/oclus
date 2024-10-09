use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
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
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_future = web::Json::<T>::from_request(req, payload);
        async move {
            match json_future.await {
                // data validation with validator
                Ok(data) => match data.validate() {
                    Ok(_) => Ok(ValidatedJson(data.into_inner())),
                    Err(error) => Err(ErrorDTO::Validation(error)),
                },
                Err(_error) => Err(ErrorDTO::WrongDataFormat),
            }
        }.boxed_local()
    }
}
