use std::{
    fmt::{Display, Formatter},
    time::Duration,
};

use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use bytes::Bytes;
use http_body_util::BodyExt;
use tower_http::trace::{DefaultOnEos, HttpMakeClassifier, TraceLayer};
use tracing::Span;

use crate::context::{REQUEST_ID_HEADER, REQUEST_ID_TRACE_KEY};

fn make_span(request: &Request) -> Span {
    // Log the request id as generated.
    let request_id = request.headers().get(REQUEST_ID_HEADER);

    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    let request_id = match request_id {
        Some(v) => v.to_str().unwrap_or_default().to_string(),
        None => String::new(),
    };

    tracing::info_span!("api request", path = %path, method = %method, {REQUEST_ID_TRACE_KEY} = ?request_id)
}

fn on_request(_req: &Request, _span: &Span) {}

fn on_response(response: &Response, latency: Duration, _span: &Span) {
    tracing::info!(
        latency = %Latency(latency),
        status = response.status().as_u16(),
        "finished processing response"
    );
}

fn on_body_chunk(chunk: &Bytes, _latency: Duration, _span: &Span) {
    let text = String::from_utf8_lossy(chunk);
    tracing::info!("response body chunk: {}", text);
}

pub fn get_trace_layer() -> TraceLayer<
    HttpMakeClassifier,
    fn(&Request) -> Span,
    fn(&Request, &Span),
    fn(&Response, Duration, &Span),
    fn(&Bytes, Duration, &Span),
    DefaultOnEos,
    (),
> {
    TraceLayer::new_for_http()
        .make_span_with(make_span as fn(&Request) -> Span)
        .on_request(on_request as fn(&Request, &Span))
        .on_failure(())
        .on_response(on_response as fn(&Response, Duration, &Span))
        .on_body_chunk(on_body_chunk as fn(&Bytes, Duration, &Span))
}

struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ms = self.0.as_millis();
        if ms > 0 {
            write!(f, "{} ms", ms)
        } else {
            write!(f, "{} us", self.0.as_micros())
        }
    }
}

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
