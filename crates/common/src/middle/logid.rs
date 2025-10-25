use crate::context;
use crate::context::{BanbridgeMetainfo, LogId};
use crate::id_gen;
use metainfo::{METAINFO, MetaInfo};
use std::cell::RefCell;
use volo::net::Address;
use volo_http::server::IntoResponse;
use volo_http::{
    body::Body, context::ServerContext, http::Uri, request::Request, response::Response,
    server::middleware::Next,
};

pub async fn log_id_middleware(
    _peer: Address,
    _uri: Uri,
    cx: &mut ServerContext,
    req: Request<Body>,
    next: Next,
) -> Response {
    let log_id = req
        .headers()
        .get(context::LOG_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| id_gen::gen_log_id().to_string());

    let mut metainfo = MetaInfo::new();
    metainfo.set_logid(LogId::from_string(log_id));

    METAINFO
        .scope(RefCell::new(metainfo), next.run(cx, req))
        .await
        .into_response()
}
