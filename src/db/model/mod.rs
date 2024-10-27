pub mod user;
pub mod group;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Entity not found in database")]
    NotFound,

    #[error("Unique constraint violation for field {0}")]
    Conflict(String),

    #[error("Unknown database error {0}")]
    Unknown(#[from] diesel::result::Error),
}