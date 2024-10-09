mod user;
mod auth;

use crate::dto::error::ErrorDTO;
use actix_web::{get, web};

pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.service(server_info);
    cfg.configure(user::config_route);
}

#[get("/")]
async fn server_info() -> Result<String, ErrorDTO> {
    Ok("Oclus API\nVersion = 0.1.0".to_string())
}
