use crate::db::{DbPool, DbPooledConnection};
use actix_web::error::BlockingError;
use actix_web::web;
use diesel::Connection;
use std::future::Future;

pub fn block_for_db<F, R>(pool: &DbPool, f: F) -> impl Future<Output = Result<R, BlockingError>>
where
    F: FnOnce(&mut DbPooledConnection) -> R + Send + 'static,
    R: Send + 'static,
{
    let pool = pool.clone();
    web::block(move || {
        // TODO: return error instead of panicking
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        f(&mut conn)
    })
}

pub fn block_for_trans_db<F, R, E>(
    pool: &DbPool,
    f: F,
) -> impl Future<Output = Result<Result<R, E>, BlockingError>>
where
    F: FnOnce(&mut DbPooledConnection) -> Result<R, E> + Send + 'static,
    R: Send + 'static,
    E: From<diesel::result::Error> + Send + 'static,
{
    let pool = pool.clone();
    web::block(move || {
        // TODO: return error instead of panicking
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        conn.transaction(|conn| f(conn))
    })
}
