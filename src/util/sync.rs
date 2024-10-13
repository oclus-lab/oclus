use crate::db;
use actix_web::error::BlockingError;
use actix_web::web;
use std::future::Future;

pub fn block_for_db<F, R>(pool: &db::Pool, f: F) -> impl Future<Output = Result<R, BlockingError>>
where
    F: FnOnce(db::PooledConnection) -> R + Send + 'static,
    R: Send + 'static,
{
    let pooled_conn = pool.get().expect("Failed to get db connection from pool");
    web::block(move || f(pooled_conn))
}
