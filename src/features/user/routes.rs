use axum::{routing::get, Router};
use super::controller;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/me", get(controller::get_me))
}