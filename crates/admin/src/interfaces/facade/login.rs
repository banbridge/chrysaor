use crate::app::AdminService;
use api::admin::v1::{LoginReq, LoginResult};
use axum::{Extension, Json};
use common::param::ApiResponse;

pub(super) async fn login_with_username(
    Extension(admin_svc): Extension<AdminService>,
    Json(req): Json<LoginReq>,
) -> ApiResponse<LoginResult> {
    todo!()
}
