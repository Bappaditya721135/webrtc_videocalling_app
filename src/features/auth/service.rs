use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use uuid::Uuid;
use tracing::info;

use sqlx::PgPool;
use super::dto::{SignupRequest, UserResponse};


pub enum SignupError {
    PasswordHash,
    Database(sqlx::Error),
}


pub async fn create_user(_pool: &PgPool, input: SignupRequest) -> Result<Option<UserResponse>, SignupError> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| SignupError::PasswordHash)?
        .to_string();


    let dummy_response: UserResponse = UserResponse {
        id: Uuid::new_v4(),
        name: input.name,
        email: input.email,
        profile_pic: input.profile_pic
    };

    Ok(Some(dummy_response))
}