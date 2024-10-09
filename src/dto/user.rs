use crate::model::user::User;
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
    pub registration_date: DateTime<Utc>,
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
            registration_date: value.registration_date.and_utc(),
        }
    }
}
