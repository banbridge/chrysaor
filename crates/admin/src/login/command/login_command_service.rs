use crate::login::command::LoginCommandService;
use api::proto_gen::v1::LoginReqDto;
use common::error::BizResult;

impl LoginCommandService {
    pub async fn login_with_username_or_id(&self, command: LoginReqDto) -> BizResult<String> {
        let token = self
            .login_domain_service
            .login_with_name_or_id(command.user_id_or_name, command.password.clone())
            .await;

        token
    }
}
