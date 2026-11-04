use axum::{routing::post, Router};
use super::controller;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(controller::signup))
        .route("/signin", post(controller::signin))
}
