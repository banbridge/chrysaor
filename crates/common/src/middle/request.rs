use crate::context;
use std::time::Instant;
use volo::net::Address;
use volo_http::body::{Body, BodyConversion};
use volo_http::{
    context::ServerContext,
    http::Uri,
    request::Request,
    response::Response,
    server::{IntoResponse, middleware::Next},
};

pub async fn request_timer_middleware(
    _peer: Address,
    _uri: Uri,
    cx: &mut ServerContext,
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();

    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let (parts, body) = req.into_parts();

    let body_bytes = body.into_string().await.unwrap_or_default();

    let log_id = context::get_or_default_log_id();

    let span = tracing::info_span!("http_request", path = %path, 
    method = %method,
    log_id = %log_id);

    let _guard = span.enter();

    tracing::info!("parts {:?}", parts);

    let body_new = Body::from(body_bytes);

    let ret = next
        .run(cx, Request::from_parts(parts, body_new))
        .await
        .into_response();

    let (parts, body) = ret.into_parts();
    let body_resp = body.into_string().await.unwrap_or_default();

    let status = parts.status;
    let cost = Instant::now().duration_since(start);

    tracing::info!(" response {status}, cost {cost:?}, body {:?}", body_resp);

    Response::from_parts(parts, Body::from(body_resp))
}
