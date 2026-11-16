mod http;
mod features;
use std::error::Error;
use tokio::net::TcpListener;
use sqlx;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>>{

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPoolOptions::new()
                .connect(&database_url)
                .await?;
    let app = http::router(pool);
    let port = std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()?;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    println!("server is running on http://localhost:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}
