use actix_web::{App, HttpServer};
mod middleware;
mod db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
