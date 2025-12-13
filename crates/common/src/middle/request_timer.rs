use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use http_body_util::BodyExt as _;

const MAX_LOG_BYTES: usize = 8 * 1024;

pub async fn request_middleware(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();
    let method = parts.method.to_string();
    let uri = parts.uri.to_string();
    let headers = parts.headers.clone();

    let bytes = body.collect().await.unwrap_or_default().to_bytes();

    let slice = if bytes.len() > MAX_LOG_BYTES {
        &bytes[..MAX_LOG_BYTES]
    } else {
        &bytes
    };
    let mut body_str = String::from_utf8_lossy(slice).to_string();
    if bytes.len() > MAX_LOG_BYTES {
        body_str.push_str(" ...[truncated]");
    }
    tracing::info!(
        "{} {} - Body ({} bytes): {}\nheaders: {:?}",
        method,
        uri,
        bytes.len(),
        body_str,
        headers
    );

    let request = Request::from_parts(parts, Body::from(bytes));

    next.run(request).await
}
