use crate::db::schema::users;
use crate::db::DbConnection;
use crate::db::model;
use chrono::NaiveDateTime;
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
    pub registered_on: NaiveDateTime,
}

#[derive(Clone, Debug)]
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
        creation_data: UserCreationData,
        db_conn: &mut DbConnection,
    ) -> Result<User, model::DbError> {
        // check for email conflict
        let email_exists = diesel::select(diesel::dsl::exists(
            users::table.filter(users::email.eq(&creation_data.email)),
        ))
        .get_result::<bool>(db_conn)?;

        if email_exists {
            return Err(model::DbError::Conflict(String::from("email")));
        }

        let id = Uuid::now_v7();
        let user = User {
            id,
            email: creation_data.email,
            username: creation_data.username,
            password: creation_data.password,
            refresh_token: creation_data.refresh_token,
            registered_on: creation_data.registered_on,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(db_conn)?;

        Ok(user)
    }

    pub fn get(id: &Uuid, db_conn: &mut DbConnection) -> Result<Option<User>, model::DbError> {
        let user = users::table.find(id).get_result(db_conn).optional()?;
        Ok(user)
    }

    pub fn get_by_email(email: &str, db_conn: &mut DbConnection) -> Result<User, model::DbError> {
        users::table
            .filter(users::email.eq(email))
            .get_result(db_conn)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => model::DbError::NotFound,
                _ => error.into(),
            })
    }

    pub fn update(
        id: &Uuid,
        update_data: &UserUpdateData,
        db_conn: &mut DbConnection,
    ) -> Result<User, model::DbError> {
        diesel::update(users::table.find(id))
            .set(update_data)
            .get_result(db_conn)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => model::DbError::NotFound,
                _ => error.into(),
            })
    }

    pub fn delete(id: &Uuid, db_conn: &mut DbConnection) -> Result<(), model::DbError> {
        let deleted = diesel::delete(users::table.find(id)).execute(db_conn)?;
        match deleted > 0 {
            true => Ok(()),
            false => Err(model::DbError::NotFound),
        }
    }
}
