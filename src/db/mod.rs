use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use std::env;

pub mod schema;

pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConnection>>;
pub type DbPooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<DbConnection>::new(database_url);

    DbPool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
