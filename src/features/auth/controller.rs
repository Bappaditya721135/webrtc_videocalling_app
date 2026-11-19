
use axum::{Json, http::StatusCode, extract::State, Extension};
use jsonwebtoken::EncodingKey;
use super::{
    dto::{ SignupRequest, SignInRequest, UserResponse, AuthResponse},
    service,
};
use sqlx::PgPool;

// SIGN UP CONTROLLER 
pub async fn signup(
    State(pool): State<PgPool>,
    Extension(jwt_key): Extension<EncodingKey>,
    Json(mut input
    ): Json<SignupRequest>) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, &'static str)> {
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
    if !(8..=128).contains(&input.password.chars().count()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must contain between 15 and 128 characters"
        ));
    }
 
    match service::create_user(&pool, input).await {
        Ok(Some(user)) => {
            let access_token = service::generate_access_token(user.id, &jwt_key)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not genrate access token"))?;
            
            Ok((
                StatusCode::CREATED,
                Json(AuthResponse {
                    access_token,
                    token_type: "Bearer".to_string(),
                    expires_in: service::ACCESS_TOKEN_TTL,
                    user,
                })
            ))
        },
        Ok(None) => Err((StatusCode::CONFLICT, "email already exist")),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "count not create account"
        )),
    }
}


// SIGN IN CONTROLLER 


pub async fn signin(
    State(pool): State<PgPool>,
    Extension(jwt_key): Extension<EncodingKey>,
    Json(mut input): Json<SignInRequest>
) ->  Result<(StatusCode, Json<AuthResponse>), (StatusCode, &'static str)> {
    input.email = input.email.trim().to_lowercase();

    // CHECK EMAIL LENGTH 
    if !(3..=254).contains(&input.email.chars().count()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email must contain between 3 and 254 characters"
        ));

    }


    // CHECK IF VALID EMAIL 
    let valid_email = match input.email.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty() && !domain.is_empty() && !domain.contains('@') && !input.email.chars().any(char::is_whitespace)
        }
        None => false,
    };

    if !valid_email {
        return Err((
            StatusCode::BAD_REQUEST,
            "Please provide a valid email"
        ));
    }

    match service::sign_in_user(&pool, input).await {
        Ok(Some(user)) => {
            let access_token = service::generate_access_token(user.id, &jwt_key)
                                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "count not generate access token"))?;

            Ok((StatusCode::OK, Json(AuthResponse {
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: service::ACCESS_TOKEN_TTL,
                user,
            })))
        },
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password"
        )),
        Err(error) => {
            eprintln!("sign in failed: {error:?}");
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "could not sign in"
        ))
        },
    }

}