use axum::{ extract::State };
use sqlx::PgPool;

pub async fn get_me(
    State(pool): State<PgPool>
) -> &'static str {

    "hello from me"
}