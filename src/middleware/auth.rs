use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};

pub async fn log_request(
    request: Request,
    next: Next,
) -> Response {
    let mut headers = request.headers().clone();
    headers.remove(header::AUTHORIZATION);

    tracing::info!(
        method = %request.method(),
        uri = %request.uri(),
        headers = ?headers,
        "Incoming request"
    );

    next.run(request).await
}