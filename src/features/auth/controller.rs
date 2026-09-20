use axum::{Json, http::StatusCode};
use super::dto::SignupRequest;

// SIGN UP CONTROLLER 
pub async fn signup(Json(input): Json<SignupRequest>) -> &'static str {
    input.name = input.name.trim().to_string();
    input.email = input.email.trim().to_lowercase();

    if input.name.is_empty() || input.name.chars().count() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Name must contain between 1 and 100 characters",
        )):
    }

    "hello from sign up"
}


// SIGN IN CONTROLLER 


pub async fn signin() -> &'static str {
    "hello from sign in"
}