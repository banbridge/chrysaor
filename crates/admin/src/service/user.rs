use crate::service::service::AdminService;
use api::admin::v1::ListUserReq;
use axum::Router;
use axum::extract::State;
use common::extract::BindAndValidate;
use common::param::{ApiResponse, ApiResult};
use tracing::instrument;

pub fn create_router() -> Router<AdminService> {
    Router::new().route("/list", axum::routing::post(list_user))
}

#[instrument(name = "list_user", skip_all)]
async fn list_user(
    State(admin_service): State<AdminService>,
    BindAndValidate(req): BindAndValidate<ListUserReq>,
) -> String {
    tracing::info!("start list_user, req{:?}", req);
    // admin_service.uc.get_user_uc().list_user(req);
    "djh".to_string()
}
