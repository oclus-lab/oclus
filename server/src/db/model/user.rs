use crate::db::schema::users;
use crate::db::{DbConnection, DieselError};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("user {0} not found")]
    UserNotFound(i64),

    #[error("email {0} already exists")]
    EmailConflict(String),

    #[error(transparent)]
    DbError(#[from] DieselError),
}

#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub registered_on: NaiveDateTime,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct UserCreationData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub registered_on: NaiveDateTime,
}

#[derive(AsChangeset, Default, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
pub struct UserUpdateData {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<Option<String>>,
    pub registered_on: Option<NaiveDateTime>,
}

impl User {
    pub fn create(
        creation_data: &UserCreationData,
        db_conn: &mut DbConnection,
    ) -> Result<User, UserError> {
        diesel::insert_into(users::table)
            .values(creation_data)
            .get_result::<User>(db_conn)
            .map_err(|err| {
                if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
                    if info.constraint_name().unwrap_or_default() == "users_email_key" {
                        return UserError::EmailConflict(creation_data.email.clone());
                    }
                }
                log::error!("database error inserting new user : {}", err);
                err.into()
            })
    }

    pub fn get(id: i64, db_conn: &mut DbConnection) -> Result<Option<Self>, UserError> {
        users::table
            .find(id)
            .get_result(db_conn)
            .optional()
            .map_err(|err| {
                log::error!("database error retrieving user {}: {}", id, err);
                err.into()
            })
    }

    pub fn get_by_email(
        email: &str,
        db_conn: &mut DbConnection,
    ) -> Result<Option<Self>, UserError> {
        users::table
            .filter(users::email.eq(email))
            .get_result(db_conn)
            .optional()
            .map_err(|err| {
                log::error!("database error retrieving user by email {}: {}", email, err);
                err.into()
            })
    }

    pub fn update(
        id: i64,
        update_data: &UserUpdateData,
        db_conn: &mut DbConnection,
    ) -> Result<Self, UserError> {
        diesel::update(users::table.find(id))
            .set(update_data)
            .get_result(db_conn)
            .map_err(|err| {
                if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
                    if let Some(email) = update_data.email.clone() {
                        if info.constraint_name().unwrap_or_default() == "users_email_key" {
                            return UserError::EmailConflict(email);
                        }
                    }
                }
                log::error!("database error updating user : {}", err);
                err.into()
            })
    }

    pub fn delete(id: i64, db_conn: &mut DbConnection) -> Result<(), UserError> {
        let deleted = diesel::delete(users::table.find(id))
            .execute(db_conn)
            .map_err(|err| {
                log::error!("database error deleting user {}: {}", id, err);
                err
            })?;
        match deleted > 0 {
            true => Ok(()),
            false => Err(UserError::UserNotFound(id)),
        }
    }
}
