use crate::db::{DbPool, DbPooledConnection};
use actix_web::error::BlockingError;
use actix_web::web;
use std::future::Future;

pub fn block_for_db<F, R>(pool: &DbPool, f: F) -> impl Future<Output = Result<R, BlockingError>>
where
    F: FnOnce(&mut DbPooledConnection) -> R + Send + 'static,
    R: Send + 'static,
{
    let pool = pool.clone();
    web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        f(&mut conn)
    })
}
