mod auth;
mod group;
mod user;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::config_routes);
    cfg.configure(user::config_routes);
    cfg.configure(group::config_routes);
}
