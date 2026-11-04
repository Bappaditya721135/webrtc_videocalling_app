use axum::{routing::get, Router};
use crate::features::auth;
use sqlx::PgPool;


pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/api/v1/auth", auth::routes::router())
        .with_state(pool)

}



async fn health() -> &'static str {
    "health checkup success"
}