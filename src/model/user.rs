use crate::db::schema::users;
use crate::db::DbConnection;
use crate::model;
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

#[derive(AsChangeset, Builder, Default, Clone, Debug)]
#[diesel(table_name = crate::db::schema::users)]
#[builder(default)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<Option<String>>,
    pub registration_date: Option<NaiveDateTime>,
}

impl UpdateUser {
    pub fn builder() -> UpdateUserBuilder {
        UpdateUserBuilder::default()
    }
}

pub fn create(creation_data: CreateUser, db_conn: &mut DbConnection) -> Result<User, model::Error> {
    db_conn.transaction(|conn| {
        // check for email conflict
        let email_exists = diesel::select(diesel::dsl::exists(
            users::table.filter(users::email.eq(&creation_data.email)),
        ))
        .get_result::<bool>(conn)?;

        if email_exists {
            return Err(model::Error::UserEmailConflict);
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

        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)?;

        Ok(user)
    })
}

pub fn get(user_id: &Uuid, db_conn: &mut DbConnection) -> Result<User, model::Error> {
    users::table
        .find(user_id)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => model::Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn get_by_email(email: &str, db_conn: &mut DbConnection) -> Result<User, model::Error> {
    users::table
        .filter(users::email.eq(email))
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => model::Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn update(
    user_id: &Uuid,
    update_data: &UpdateUser,
    db_conn: &mut DbConnection,
) -> Result<User, model::Error> {
    diesel::update(users::table.find(user_id))
        .set(update_data)
        .get_result(db_conn)
        .map_err(|error| match error {
            diesel::result::Error::NotFound => model::Error::UserNotFound,
            _ => error.into(),
        })
}

pub fn delete(user_id: &Uuid, db_conn: &mut DbConnection) -> Result<(), model::Error> {
    let deleted = diesel::delete(users::table.find(user_id)).execute(db_conn)?;
    match deleted > 0 {
        true => Ok(()),
        false => Err(model::Error::UserNotFound),
    }
}
