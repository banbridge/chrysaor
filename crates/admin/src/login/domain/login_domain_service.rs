use common::error::AppErrorBizBuilder;
use common::jwt::Principal;
use common::{check::require_non_empty, error::AppResult, password::BcryptEncoder};
use faststr::FastStr;

impl super::LoginDomainService {
    pub async fn login_with_name_or_id(
        &self,
        name_or_id: FastStr,
        password: FastStr,
    ) -> AppResult<String> {
        require_non_empty(&name_or_id, "name_or_id is empty")?;
        require_non_empty(&password, "password is empty")?;

        let user = self
            .repo_factory
            .get_user_repo()
            .get_user(name_or_id)
            .await?;

        let mc = BcryptEncoder.matches(password, user.password.unwrap_or_default())?;

        if !mc {
            return Err(AppErrorBizBuilder::invalid_param(
                "password is incorrect".into(),
            ));
        }

        let principal = Principal {
            user_id: user.user_id.unwrap_or_default().into(),
            username: user.username.unwrap_or_default().into(),
        };

        let token = self.jwt.encode(principal)?;

        Ok(token)
    }
}
