use crate::error::AppErrorBizBuilder;
use crate::param::ApiResponse;
use volo_http::context::ServerContext;
use volo_http::response::Response;
use volo_http::server::IntoResponse;

pub async fn failed_fallback_middleware() -> Response {
    ApiResponse::<()>::err(AppErrorBizBuilder::request_not_found(
        "request not found".into(),
    ))
    .into_response()
}

pub fn timeout_middleware(_: &ServerContext) -> ApiResponse<()> {
    ApiResponse::err(AppErrorBizBuilder::request_timeout("timeout".into()))
}
