use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

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
