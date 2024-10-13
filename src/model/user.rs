use crate::db::schema::users;
use chrono::NaiveDateTime;
use derive_builder::Builder;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub registration_date: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub registration_date: NaiveDateTime,
}

#[derive(AsChangeset, Builder, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct UpdateUser {
    #[builder(default)]
    pub email: Option<String>,
    #[builder(default)]
    pub username: Option<String>,
    #[builder(default)]
    pub password: Option<String>,
    #[builder(default)]
    pub refresh_token: Option<Option<String>>,
    #[builder(default)]
    pub registration_date: Option<NaiveDateTime>,
}

impl UpdateUser {
    pub fn builder() -> UpdateUserBuilder {
        UpdateUserBuilder::default()
    }
}

pub fn create_user(creation_data: CreateUser, db_conn: &mut PgConnection) -> Result<User, Error> {
    // check for email conflict
    let email_already_exists = users::table
        .filter(users::email.eq(&creation_data.email))
        .first::<User>(db_conn)
        .optional()?
        .is_some();

    if email_already_exists {
        return Err(Error::EmailConflict);
    }

    let id = Uuid::now_v7();
    let user = User {
        id,
        email: creation_data.email,
        username: creation_data.username,
        password: creation_data.password,
        refresh_token: creation_data.refresh_token,
        registration_date: creation_data.registration_date,
    };

    diesel::insert_into(users::table).values(&user).execute(db_conn)?;

    Ok(user)
}

pub fn read_user(user_id: &Uuid, db_conn: &mut PgConnection) -> Result<User, Error> {
    users::table
        .find(user_id)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn read_user_by_email(email: &str, db_conn: &mut PgConnection) -> Result<User, Error> {
    users::table
        .filter(users::email.eq(email))
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn update_user(user_id: &Uuid, update_data: &UpdateUser, db_conn: &mut PgConnection) -> Result<User, Error> {
    diesel::update(users::table.find(user_id))
        .set(update_data)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn delete_user(user_id: &Uuid, db_conn: &mut PgConnection) -> Result<(), Error> {
    let deleted = diesel::delete(users::table.find(user_id)).execute(db_conn)?;
    match deleted > 0 {
        true => Ok(()),
        false => Err(Error::UserNotFound),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("User not found in database")]
    UserNotFound,

    #[error("Email already exists in database")]
    EmailConflict,

    #[error("Database error")]
    Database,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        log::error!("Database error: {}", value);
        Error::Database
    }
}
