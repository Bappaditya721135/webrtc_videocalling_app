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
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("server is running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
