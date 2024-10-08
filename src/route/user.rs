use crate::dto::error::ErrorDetail;
use actix_web::{post, web};

pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}

#[post("/register")]
pub async fn register() -> Result<String, ErrorDetail> {
    todo!()
}
