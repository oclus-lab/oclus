use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupPublicDto {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupDto {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_on: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct CreateGroupRequest {
    pub name: String
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
}