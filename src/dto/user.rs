use crate::db::model::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Clone, Debug)]
pub struct PublicProfile {
    pub id: Uuid,
    pub username: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct PrivateProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub registered_on: DateTime<Utc>,
}

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct UpdateProfileRequest {
    #[validate(email)]
    pub email: Option<String>,
    pub username: Option<String>,
}

impl From<User> for PublicProfile {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}

impl From<User> for PrivateProfile {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            registered_on: value.registered_on.and_utc(),
        }
    }
}
