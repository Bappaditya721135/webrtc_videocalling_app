use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
    PasswordVerifier,
    PasswordHash,
};
use uuid::Uuid;
use tracing::info;

use sqlx::PgPool;
use super::dto::{SignupRequest, UserResponse, SignInRequest};

#[derive(Debug)]
#[derive(sqlx::FromRow)]
struct UserWithPassword {
    id: uuid::Uuid,
    name: String,
    email: String,
    password_hash: String,
    profile_pic: Option<String>,
}

#[derive(Debug)]
pub enum AuthError {
    EmailAlreadyExists,
    InvalidCredentials,
    PasswordHash,
    Database(sqlx::Error),
}


pub async fn create_user(pool: &PgPool, input: SignupRequest) -> Result<Option<UserResponse>, AuthError> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| AuthError::PasswordHash)?
        .to_string();


    let user = sqlx::query_as::<_,UserResponse>(
        r#"
        INSERT INTO Users (name, email, password_hash, profile_pic)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (email) DO NOTHING
        RETURNING id, name, email, profile_pic
        "#
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&input.profile_pic)
    .fetch_optional(pool)
    .await
    .map_err(AuthError::Database)?;

    Ok(user)
}


pub async fn sign_in_user(pool: &PgPool, input: SignInRequest) -> Result<Option<UserResponse>, AuthError> {

    println!("email: {:?}", input.email);
    println!("password: {:?}",input.password);
    let user = sqlx::query_as::<_, UserWithPassword>(
        r#"
        SELECT id, name, password_hash, profile_pic FROM users WHERE email = &1
        "#
    )
    .bind(&input.email)
    .fetch_optional(pool)
    .await
    .map_err(AuthError::Database)?;

    println!("user: {:?}", user);

    let Some(user) = user else {
        return Ok(None);
    };

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AuthError::PasswordHash)?;

    if Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed_hash)
        .is_err() 
    {
        return Ok(None);
    }

    Ok(Some(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        profile_pic: user.profile_pic,
    }))

}