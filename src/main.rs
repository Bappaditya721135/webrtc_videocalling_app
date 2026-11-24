mod http;
mod middleware;
mod features;
use std::error::Error;
use tokio::net::TcpListener;
use sqlx;
use axum::Extension;
use jsonwebtoken::EncodingKey;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>>{

    dotenvy::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET")?;
    if jwt_secret.len() < 32 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "JWT_SECRET must contain at least 32 bytes",).into());
    }
    let jwt_key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPoolOptions::new()
                .connect(&database_url)
                .await?;
    let app = http::router(pool)
                .layer(Extension(jwt_key));
    let port = std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()?;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    println!("server is running on http://localhost:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}
