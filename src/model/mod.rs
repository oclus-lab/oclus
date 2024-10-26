pub mod user;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Entity not found in database")]
    NotFound,

    #[error("Unique constraint violation for field {0}")]
    Conflict(String),

    #[error("Database error")]
    Database(#[from] diesel::result::Error),
}