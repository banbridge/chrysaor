use crate::{conf, domain, util};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminService {
    pub config: conf::AppConf,
    pub uc: Arc<dyn domain::IAdminUC>,
    pub jwt: Arc<util::jwt::JWT>,
}

impl AdminService {
    pub fn new(config: &conf::AppConf, uc: Arc<dyn domain::IAdminUC>) -> Self {
        AdminService {
            config: config.clone(),
            uc,
            jwt: Arc::new(util::jwt::JWT::new(&config)),
        }
    }
}
