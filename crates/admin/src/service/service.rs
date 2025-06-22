use crate::{conf, domain, util};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminService {
    pub config: Arc<conf::AppConf>,
    pub uc: Arc<dyn domain::IAdminUC>,
    pub jwt: Arc<util::jwt::JWT>,
}

impl AdminService {
    pub fn new(config: Arc<conf::AppConf>, uc: Arc<dyn domain::IAdminUC>) -> Self {
        AdminService {
            config: Arc::clone(&config),
            uc,
            jwt: Arc::new(util::jwt::JWT::new(Arc::clone(&config))),
        }
    }
}
