use axum::{Json, http::StatusCode};
use super::dto::SignupRequest;

// SIGN UP CONTROLLER 
pub async fn signup(Json(mut input): Json<SignupRequest>) -> Result<&'static str, (StatusCode, &'static str)> {
    input.name = input.name.trim().to_string();
    input.email = input.email.trim().to_lowercase();

    if input.name.is_empty() || input.name.chars().count() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Name must contain between 1 and 100 characters",
        ));
    }

    // CHECK EMAIL LENGTH 
    if !(3..=254).contains(&input.email.chars().count()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email must contain between 3 and 254 characters"
        ));
    }


    let valid_email = match input.email.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty() && !domain.is_empty() && !domain.contains('@') && !input.email.chars().any(char::is_whitespace)
        }
        None => false,
    };

    if !valid_email {
        return Err((
            StatusCode::BAD_REQUEST,
            "Enter a valid email"
        ));
    }

    // PASSWORD LENGTH VALIDATION 
    if !(15..=128).contains(&input.password.chars().count()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must contain between 15 and 128 characters"
        ));
    }
 
    Ok("Sign up validation passed")
}


// SIGN IN CONTROLLER 


pub async fn signin() -> &'static str {
    "hello from sign in"
}