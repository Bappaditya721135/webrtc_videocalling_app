use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub profile_pic: Option<String>
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String
}


#[derive(Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub profile_pic: Option<String>
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserResponse,
}