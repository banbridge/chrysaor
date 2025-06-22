use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use common::context::{BanbridgeMetainfo, LogId};
use common::error::BizError;
use common::{context, id_gen};
use metainfo::{METAINFO, MetaInfo};
use std::cell::RefCell;

use crate::service::service::AdminService;

pub async fn jwt_auth(
    State(state): State<AdminService>,
    req: Request,
    next: Next,
) -> Result<Response, BizError> {
    let token_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(BizError::unauthenticated("authorization header is empty"))?;

    let token = token_header
        .to_str()
        .map_err(|err| {
            tracing::error!("authorization header to_str error: {:?}", err);
            BizError::unauthenticated("authorization header is not a valid string")
        })?
        .strip_prefix("Bearer ")
        .ok_or(BizError::unauthenticated(
            "authorization header must be Bearer type",
        ))?;

    tracing::info!("jwt_auth token: {:?}", token);

    let principal = state.jwt.decode(token)?;

    tracing::info!("jwt_auth principal: {:?}", principal);

    // 继续处理请求
    Ok(next.run(req).await)
}

pub async fn request_middleware(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();
    let method = parts.method.to_string();
    let uri = parts.uri.to_string();
    let headers = parts.headers.clone();

    // 读取body（限制为8KB）

    // 打印基本信息
    tracing::info!(
        "{} {} - Body ({} bytes): {} \n header{:?}",
        method,
        uri,
        10,
        "",
        headers
    );

    let request = Request::from_parts(parts, body);

    // 处理请求
    let response = next.run(request).await;

    // 记录响应信息
    let (resp_parts, resp_body) = response.into_parts();
    let resp_status = resp_parts.status;

    tracing::info!(
        "request_middleware response status:{} {:?}",
        resp_status,
        resp_body
    );
    Response::from_parts(resp_parts, resp_body)
}

pub async fn request_timer(req: Request, next: Next) -> Response {
    let log_id = req
        .headers()
        .get(context::LOG_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| id_gen::gen_log_id().to_string());

    let mut metainfo = MetaInfo::new();
    metainfo.set_logid(LogId::from_string(log_id));

    METAINFO.scope(RefCell::new(metainfo), next.run(req)).await
}
