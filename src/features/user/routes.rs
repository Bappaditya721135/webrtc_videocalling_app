use axum::{routing::get, Router, middleware};
use super::controller;
use sqlx::PgPool;
use crate::middleware::require_auth;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/me",
             get(controller::get_me)
             .route_layer(middleware::from_fn(require_auth))
            )
}