use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub mod schema;
pub mod model;

pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConnection>>;
pub type DbPooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

pub type DieselError = diesel::result::Error;

pub fn init_conn() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<DbConnection>::new(database_url);

    let pool = DbPool::builder()
        .build(manager)
        .expect("Failed to create pool");
    
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

    pool
}
