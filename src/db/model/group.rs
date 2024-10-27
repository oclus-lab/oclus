use crate::db::model;
use crate::db::model::user::User;
use crate::db::model::DbError;
use crate::db::schema::groups;
use crate::db::DbConnection;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_on: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct GroupCreationData {
    pub name: String,
    pub owner_id: Uuid,
    pub created_on: NaiveDateTime,
}

#[derive(AsChangeset, Default, Clone, Debug)]
#[diesel(table_name = crate::db::schema::groups)]
pub struct GroupUpdateData {
    pub name: Option<String>,
    pub owner_id: Option<Uuid>,
    pub created_on: Option<NaiveDateTime>,
}

impl Group {
    pub fn create(
        creation_data: GroupCreationData,
        db_conn: &mut DbConnection,
    ) -> Result<Self, DbError> {
        // check if the owner exists
        User::get(&creation_data.owner_id, db_conn)?.ok_or(DbError::NotFound)?;

        let group = Group {
            id: Uuid::now_v7(),
            name: creation_data.name,
            owner_id: creation_data.owner_id,
            created_on: creation_data.created_on,
        };

        diesel::insert_into(groups::table)
            .values(&group)
            .execute(db_conn)?;
        Ok(group)
    }

    pub fn read(id: &Uuid, db_conn: &mut DbConnection) -> Result<Option<Self>, DbError> {
        let group = groups::table
            .find(id)
            .get_result::<Group>(db_conn)
            .optional()?;
        Ok(group)
    }

    pub fn update(
        update_data: &GroupUpdateData,
        db_conn: &mut DbConnection,
    ) -> Result<Self, model::DbError> {
        todo!()
    }

    pub fn delete(id: &Uuid, db_conn: &mut DbConnection) -> Result<(), model::DbError> {
        let deleted = diesel::delete(groups::table.find(id)).execute(db_conn)?;
        match deleted {
            0 => Err(DbError::NotFound),
            _ => Ok(()),
        }
    }
}
