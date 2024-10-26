use crate::middleware::auth::strong::StrongAuthenticator;
use crate::middleware::auth::Authenticator;
use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::{web, App};
use crate::db::DbPool;

pub mod db; // public for integration tests
pub mod dto;
mod middleware;
mod model;
mod route;
mod util;

pub fn app(db_pool: DbPool) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .configure(route::config)
        .app_data(web::Data::new(db_pool.clone()))
        .wrap(StrongAuthenticator::new(db_pool))
        .wrap(Authenticator)
}
