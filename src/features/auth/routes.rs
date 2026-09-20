use axum::{routing::post, Router};
use super::controller;

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(controller::signup))
        .route("/signin", post(controller::signin))
}
