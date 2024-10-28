use crate::db::model::user::UserError;
use crate::db::schema::groups;
use crate::db::DbConnection;
use crate::db::DieselError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum GroupError {
    #[error("group {0} does not exist")]
    GroupNotFound(i64),

    #[error("group owner {0} does not exist")]
    OwnerNotFound(i64),

    #[error(transparent)]
    DbError(#[from] DieselError),
}

#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_on: NaiveDateTime,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct GroupCreationData {
    pub name: String,
    pub owner_id: i64,
    pub created_on: NaiveDateTime,
}

#[derive(AsChangeset, Default, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct GroupUpdateData {
    pub name: Option<String>,
    pub owner_id: Option<i64>,
    pub created_on: Option<NaiveDateTime>,
}

impl Group {
    pub fn create(
        creation_data: &GroupCreationData,
        db_conn: &mut DbConnection,
    ) -> Result<Self, GroupError> {
        diesel::insert_into(groups::table)
            .values(creation_data)
            .get_result::<Group>(db_conn)
            .map_err(|err| {
                if let DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) =
                    &err
                {
                    if info.column_name().unwrap_or_default() == "owner_id" {
                        return GroupError::OwnerNotFound(creation_data.owner_id);
                    }
                }
                log::error!("database error inserting new group: {}", err);
                err.into()
            })
    }

    pub fn get(id: i64, db_conn: &mut DbConnection) -> Result<Option<Self>, UserError> {
        groups::table
            .find(id)
            .get_result::<Group>(db_conn)
            .optional()
            .map_err(|err| {
                log::error!("database error retrieving group {}: {}", id, err);
                err.into()
            })
    }

    pub fn update(
        id: i64,
        update_data: &GroupUpdateData,
        db_conn: &mut DbConnection,
    ) -> Result<Self, GroupError> {
        diesel::update(groups::table.find(id))
            .set(update_data)
            .get_result::<Group>(db_conn)
            .map_err(|err| {
                // check if error is due to new owner not found
                if let Some(owner_id) = update_data.owner_id {
                    if let DieselError::DatabaseError(
                        DatabaseErrorKind::ForeignKeyViolation,
                        info,
                    ) = &err
                    {
                        if info.column_name().unwrap_or_default() == "owner_id" {
                            return GroupError::OwnerNotFound(owner_id);
                        }
                    }
                }
                log::error!("database error updating group: {}", err);
                err.into()
            })
    }

    pub fn delete(id: i64, db_conn: &mut DbConnection) -> Result<(), GroupError> {
        let deleted = diesel::delete(groups::table.find(id))
            .execute(db_conn)
            .map_err(|err| {
                log::error!("database error deleting group {}: {}", id, err);
                err
            })?;

        match deleted {
            0 => Err(GroupError::GroupNotFound(id)),
            _ => Ok(()),
        }
    }
}
