use crate::infrastructure::repository::DBFactoryTrait;
use crate::infrastructure::support::UserClaim;
use common::error::{AppErrorBuilt, AppResult};
use common::jwt::JWTManager;
use common::password::BcryptEncoder;
use faststr::FastStr;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(name="login_domain_service", binds=[Self::into_service])]
pub(crate) struct LoginDomainService {
    #[di(name = "db_factory")]
    db_factory: Arc<dyn DBFactoryTrait>,

    #[di(name = "jwt_manager")]
    jwt_manager: Arc<JWTManager>,
}

impl LoginDomainService {
    fn into_service(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub async fn login_with_username(
        &self,
        username: FastStr,
        password: FastStr,
    ) -> AppResult<String> {
        let user_info = self
            .db_factory
            .get_user_repository()
            .get_user(username)
            .await?;

        let ok = BcryptEncoder.matches(password, user_info.password.into())?;

        if !ok {
            return Err(AppErrorBuilt::password_invalid(
                "password or username is wrong".to_string(),
            ));
        }

        let jwt_token = self.jwt_manager.encode::<UserClaim>(UserClaim::new(
            FastStr::from_string(user_info.user_id),
            FastStr::from_string(user_info.username),
        ))?;

        Ok(jwt_token)
    }
}
