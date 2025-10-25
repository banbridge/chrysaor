use crate::helper::GLOBAL_JWT;
use crate::login::command::LoginCommandService;
use crate::service::AdminService;
use api::proto_gen;
use common::extract::BindAndValidate;
use common::jwt::Principal;
use common::param::{ApiResponse, ApiResult};
use sea_orm::Iden;
use std::sync::Arc;
use volo_http::Router;
use volo_http::server::route::post;
use volo_http::utils::Extension;

#[derive(Clone)]
#[rudi::Singleton(async, name = "login_controller", binds=[Self::into_login_controller])]
pub struct LoginController {
    #[di(name = "login_command_service")]
    pub login_command_service: Arc<LoginCommandService>,
}

impl LoginController {
    /// 获取登录控制器的路由配置
    ///
    /// # 返回值
    /// 返回配置好的 `Router` 实例，包含登录相关的路由
    pub fn get_routers_v1(&self) -> Router {
        // 正确的方法引用：使用 Self::login 而不是 self.login
        Router::new()
            .route("/login", post(login))
            .route("/get", post(user_info))
    }

    fn into_login_controller(self) -> Arc<Self> {
        Arc::new(self)
    }
}

async fn login(
    Extension(admin_service): Extension<Arc<AdminService>>,
    BindAndValidate(req): BindAndValidate<proto_gen::v1::LoginReqDto>,
) -> ApiResult<proto_gen::v1::LoginResultDto> {
    tracing::info!("login login req: {:?}", req);
    let token = admin_service
        .login_controller
        .login_command_service
        .login_with_username_or_id(req)
        .await?;

    Ok(ApiResponse::ok_with_data(proto_gen::v1::LoginResultDto {
        token: token.into(),
    }))
}

async fn user_info(Extension(user): Extension<Principal>) -> ApiResult<String> {
    tracing::info!("user info: {:?}", user);

    Ok(ApiResponse::ok_with_data("success".to_string()))
}
