use crate::service::service::AdminService;
use axum::Router;
use common::error::BizError;
use common::param;
use faststr::FastStr;

pub mod service;
mod user;

pub fn create_router() -> Router<AdminService> {
    Router::new()
        .nest(
            "/api",
            Router::new().nest("/v1", Router::new().nest("/user", user::create_router())),
        )
        .fallback(async || -> param::ApiResult<()> {
            tracing::warn!("unable to create API router");
            Err(BizError::new(
                400,
                FastStr::from("not found"),
                10032,
                FastStr::from("NotFound"),
            ))
        })
}
