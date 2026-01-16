use crate::app::AdminService;
use api::admin::v1::LoginReq;
use axum::{Extension, Json};
use common::param::ApiResponse;

pub(super) async fn login(
    Extension(admin_svc): Extension<AdminService>,
    Json(req): Json<LoginReq>,
) -> ApiResponse<String> {
    todo!()
}
