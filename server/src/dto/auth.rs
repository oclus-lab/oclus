use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenPair {
    pub auth_token: String,
    pub refresh_token: String,
}
