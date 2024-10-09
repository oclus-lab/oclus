use crate::db::schema::users;
use chrono::NaiveDate;
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
    pub registration_date: NaiveDate,
}

#[derive(Clone, Debug)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub registration_date: NaiveDate,
}

#[derive(AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<Option<String>>,
    pub registration_date: Option<NaiveDate>,
}

impl Default for UpdateUser {
    fn default() -> Self {
        Self {
            email: None,
            username: None,
            password: None,
            refresh_token: None,
            registration_date: None,
        }
    }
}

pub fn create_user(creation_data: CreateUser, db_conn: &mut PgConnection) -> Result<User, Error> {
    let id = Uuid::now_v7();
    let user = User {
        id,
        email: creation_data.email,
        username: creation_data.username,
        password: creation_data.password,
        refresh_token: creation_data.refresh_token,
        registration_date: creation_data.registration_date,
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(db_conn)?;

    Ok(user)
}

pub fn read_user(user_id: Uuid, db_conn: &mut PgConnection) -> Result<User, Error> {
    users::table
        .find(user_id)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => Error::DieselError(error),
        })
}

pub fn read_user_by_email(email: &str, db_conn: &mut PgConnection) -> Result<User, Error> {
    users::table
        .filter(users::email.eq(email))
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => Error::DieselError(error),
        })
}

pub fn update_user(
    user_id: &Uuid,
    update_data: &UpdateUser,
    db_conn: &mut PgConnection,
) -> Result<User, Error> {
    diesel::update(users::table.find(user_id))
        .set(update_data)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => Error::UserNotFound,
            _ => Error::DieselError(error),
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

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
}
