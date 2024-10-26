use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use oclus_server::db::{DbConnection, DbPool};
use std::env;

pub fn setup_test_db() -> DbPool {
    dotenv().ok();

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
