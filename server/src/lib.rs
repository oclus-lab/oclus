use crate::db::DbPool;
use crate::middleware::auth::strong::StrongAuthenticator;
use crate::middleware::auth::Authenticator;
use actix_http::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{web, App};

pub mod db; // public for tests
pub mod dto;
mod middleware;
mod route;
mod util;

pub fn app(
    db_pool: DbPool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .configure(route::config)
        .app_data(web::Data::new(db_pool.clone()))
        .wrap(StrongAuthenticator::new(db_pool))
        .wrap(Authenticator)
        .wrap(Logger::default())
}
