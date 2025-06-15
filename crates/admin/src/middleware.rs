use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use common::error::BizError;

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
