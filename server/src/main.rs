use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use oclus_server::app::service;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv()?;

    // setup database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migration").run(&db_pool).await?;

    let server = HttpServer::new(move || {
        App::new()
            .configure(service::configure)
            .app_data(Data::new(db_pool.clone()))
            .wrap(Logger::default())
    });

    let bind_addr = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
    server.bind(bind_addr)?.run().await?;

    Ok(())
}
