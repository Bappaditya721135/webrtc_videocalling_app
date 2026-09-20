mod http;
mod features;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>>{
    let app = http::router();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("server is running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
