use crate::middleware::auth::strong::StrongAuthMiddlewareFactory;
use crate::middleware::auth::AuthMiddlewareFactory;
use actix_web::{web, App, HttpServer};

mod db;
mod dto;
mod middleware;
mod model;
mod route;
mod util;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = db::establish_connection();

    HttpServer::new(move || {
        let auth_mw = AuthMiddlewareFactory;
        let strong_auth_mw = StrongAuthMiddlewareFactory {
            db_pool: db_pool.clone(),
        };

        App::new()
            .configure(route::config_routes)
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(strong_auth_mw)
            .wrap(auth_mw)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
