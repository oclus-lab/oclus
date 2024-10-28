use actix_http::body::MessageBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use oclus_server::app;
use oclus_server::db::{DbConnection, DbPool};
use std::env;

pub async fn test_app() -> impl Service<
    actix_http::Request,
    Response = ServiceResponse<impl MessageBody>,
    Error = actix_web::Error,
> {
    dotenv().ok();
    let db_pool = init_db_conn();

    test::init_service(app(db_pool)).await
}

fn init_db_conn() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<DbConnection>::new(database_url);
    let pool = DbPool::builder()
        .build(manager)
        .expect("Failed to create pool");

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    pool
}
