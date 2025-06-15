use crate::obj::ListUserReq;
use crate::service::service::AdminService;
use axum::extract::State;
use axum::{Router, debug_handler};
use common::extract::BindAndValidate;
use common::param::{ApiResponse, ApiResult};
use tracing::instrument;

pub fn create_router() -> Router<AdminService> {
    Router::new().route("/list", axum::routing::post(list_user))
}

#[debug_handler]
#[instrument(name = "list_user", skip_all)]
async fn list_user(
    State(admin_service): State<AdminService>,
    BindAndValidate(req): BindAndValidate<ListUserReq>,
) -> ApiResult<String> {
    tracing::info!("start list_user");
    //admin_service.uc.get_user_uc().list_user(req);
    Ok(ApiResponse::ok_with_data("ok".to_string()))
}
