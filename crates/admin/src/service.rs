use crate::helper::AppConf;
use crate::login::LoginController;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(async, name = "admin_service", binds=[Self::into_admin_service])]
pub struct AdminService {
    #[di(name = "config")]
    pub config: Arc<AppConf>,

    #[di(name = "login_controller")]
    pub login_controller: Arc<LoginController>,
}

impl AdminService {
    fn into_admin_service(self) -> Arc<Self> {
        Arc::new(self)
    }
}
