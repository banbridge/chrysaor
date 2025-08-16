use axum::{extract::Request, response::IntoResponse};

use crate::{conf, domain, util};
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(async, name = "admin_service")]
pub struct AdminService {
    #[di(name = "config")]
    pub config: Arc<conf::AppConf>,
    #[di(name = "admin_uc")]
    pub uc: Arc<dyn domain::IAdminUC>,
    #[di(name = "jwt")]
    pub jwt: Arc<util::jwt::JWT>,
}
