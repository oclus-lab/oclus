use actix_web::{App, HttpServer};

mod db;
mod dto;
mod middleware;
mod route;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| App::new().configure(route::config_route))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
