use common::error::AppResult;
use faststr::FastStr;

use crate::domain::login_domain;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(async,name="login_app_service", binds=[Self::into_service])]
pub struct LoginAppService {
    #[di(name = "login_domain_service")]
    login_domain_service: Arc<login_domain::LoginDomainService>,
}

impl LoginAppService {
    pub fn into_service(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub async fn login_with_username(
        &self,
        username: FastStr,
        password: FastStr,
    ) -> AppResult<String> {
        self.login_domain_service
            .login_with_username(username, password)
            .await
    }
}
