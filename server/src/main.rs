use actix_web::HttpServer;
use dotenv::dotenv;
use oclus_server::{app, db};

#[actix_web::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let db_pool = db::init_conn();
    HttpServer::new(move || app(db_pool.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
