use actix_web::{web, App, HttpServer};

mod db;
mod dto;
mod middleware;
mod route;
mod util;
mod model;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .configure(route::config_routes)
            .app_data(web::Data::new(db_pool.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
