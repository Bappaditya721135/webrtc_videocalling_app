use axum::{routing::get, Router};
use crate::features::auth;


pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/api/v1/auth", auth::routes::router())

}



async fn health() -> &'static str {
    "health checkup success"
}