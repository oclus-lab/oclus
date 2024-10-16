mod user;
mod auth;
mod group;

use crate::dto::error::ErrorDTO;
use actix_web::{get, web};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(server_info);
    cfg.configure(auth::config_routes);
    cfg.configure(user::config_routes);
}

#[get("/")]
async fn server_info() -> Result<String, ErrorDTO> {
    Ok("Oclus API\nVersion = 0.1.0".to_string())
}
