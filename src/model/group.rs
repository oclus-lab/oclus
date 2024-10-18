use crate::db::schema::{groups, users};
use crate::model;
use crate::model::user::User;
use derive_builder::Builder;
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::DbConnection;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
}

#[derive(Clone, Debug)]
pub struct CreateGroup {
    pub name: String,
    pub owner_id: Uuid,
}

#[derive(AsChangeset, Builder, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct UpdateGroup {
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub owner_id: Option<Uuid>,
}

impl UpdateGroup {
    pub fn builder() -> UpdateGroupBuilder {
        UpdateGroupBuilder::default()
    }
}

pub fn create(
    creation_data: CreateGroup,
    db_conn: &mut DbConnection,
) -> Result<Group, model::Error> {
    db_conn.transaction(|conn| {
        // check if owner exists
        let owner_exists = diesel::select(diesel::dsl::exists(
            users::table.find(creation_data.owner_id),
        ))
        .get_result::<bool>(conn)?;
        
        if !owner_exists {
            return Err(model::Error::UserNotFound);
        }

        let new_group = Group {
            id: Uuid::now_v7(),
            name: creation_data.name,
            owner_id: creation_data.owner_id,
        };

        diesel::insert_into(groups::table)
            .values(&new_group)
            .execute(conn)?;

        Ok(new_group)
    })
}

pub fn get(group_id: &Uuid, db_conn: &mut DbConnection) -> Result<Group, model::Error> {
    let group = groups::table.find(group_id).first(db_conn)?;
    Ok(group)
}

pub fn update(
    update_data: &UpdateGroup,
    db_conn: &mut DbConnection,
) -> Result<Group, model::Error> {
    db_conn.transaction(|conn| {
        if let Some(new_owner_id) = update_data.owner_id {
            // check if owner exists
            users::table
                .find(new_owner_id)
                .first::<User>(conn)
                .optional()?
                .ok_or(model::Error::UserNotFound)?;
        }

        let group = diesel::update(groups::table)
            .set(update_data)
            .get_result(conn)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => model::Error::GroupNotFound,
                _ => error.into(),
            })?;

        Ok(group)
    })
}

pub fn delete(group_id: &Uuid, db_conn: &mut DbConnection) -> Result<(), model::Error> {
    let deleted = diesel::delete(groups::table)
        .filter(groups::id.eq(group_id))
        .execute(db_conn)?;

    match deleted {
        0 => Err(model::Error::GroupNotFound),
        _ => Ok(()),
    }
}
