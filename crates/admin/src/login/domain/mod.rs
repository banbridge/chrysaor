use common::jwt::JWT;
use infra::repo::{self};
use std::sync::Arc;

pub mod login_domain_service;

#[derive(Clone)]
#[rudi::Singleton(async, name = "login_domain_service", binds=[Self::into_login_domain_service])]
pub(crate) struct LoginDomainService {

    #[di(name = "repo_factory")]
    pub repo_factory: Arc<dyn repo::IRepoFactory>,

    #[di(name = "mysql_db")]
    pub db: Arc<sea_orm::DatabaseConnection>,

    #[di(name = "jwt")]
    pub jwt: Arc<JWT>,
}

impl LoginDomainService {
    pub fn into_login_domain_service( self) -> Arc<Self> {
        Arc::new(self)
    }
}
