use crate::login::domain::LoginDomainService;
use std::sync::Arc;

mod login_command_service;

#[derive(Clone)]
#[rudi::Singleton(async, name = "login_command_service", binds=[Self::into_login_command_service])]
pub(crate) struct LoginCommandService {
    #[di(name = "login_domain_service")]
    pub login_domain_service: Arc<LoginDomainService>,

}

impl LoginCommandService {
    fn into_login_command_service(self) -> std::sync::Arc<Self> {
        std::sync::Arc::new(self)
    }
}
