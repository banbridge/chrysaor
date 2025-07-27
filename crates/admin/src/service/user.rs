use crate::service::service::AdminService;
use api::admin::{ListUserReq, ListUserResult, LoginReq, LoginResult};
use axum::Router;
use axum::extract::State;
use common::error::BizResult;
use common::extract::BindAndValidate;
use common::param::{ApiResponse, ApiResult};
use tracing::instrument;

pub fn create_router() -> Router<AdminService> {
    Router::new()
        .route("/list", axum::routing::post(list_user))
        .route("/login", axum::routing::post(login))
}

#[instrument(name = "list_user", skip_all)]
async fn list_user(
    State(admin_service): State<AdminService>,
    BindAndValidate(req): BindAndValidate<ListUserReq>,
) -> BizResult<ApiResponse<ListUserResult>> {
    tracing::info!("start list_user, req{:?}", req);
    let list_user_output = admin_service.uc.get_user_uc().list_user(req.into()).await?;
    Ok(ApiResponse::ok_with_data(list_user_output.into()))
}

#[instrument(name = "login", skip_all)]
async fn login(
    State(admin_service): State<AdminService>,
    BindAndValidate(req): BindAndValidate<LoginReq>,
) -> BizResult<ApiResponse<LoginResult>> {
    tracing::info!("start login, req{:?}", req);

    let token = admin_service.uc.get_user_uc().login(req.into()).await?;

    Ok(ApiResponse::ok_with_data(LoginResult { token }))
}
