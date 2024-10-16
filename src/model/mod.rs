pub mod user;
pub mod group;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("User not found in database")]
    UserNotFound,

    #[error("Email already exists in database")]
    UserEmailConflict,

    #[error("Group not found in database")]
    GroupNotFound,

    #[error("Database error")]
    Database,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        log::error!("Database error: {}", value);
        Error::Database
    }
}