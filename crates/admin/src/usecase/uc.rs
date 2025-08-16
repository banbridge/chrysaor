use crate::data::AdminRepo;
use crate::domain::{IAdminUC, IUserUC};
use crate::{data, util};
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(async,name = "admin_uc", binds=[Self::into_admin_uc])]
pub struct AdminUsecase {
    #[di(name = "admin_repo")]
    pub admin_repo: Arc<AdminRepo>,

    #[di(name = "user_uc")]
    pub user_uc: Arc<dyn IUserUC>,

    #[di(name = "data")]
    pub data: Arc<data::Data>,

    #[di(name = "jwt")]
    pub jwt: Arc<util::jwt::JWT>,
}

impl AdminUsecase {
    fn into_admin_uc(self) -> Arc<dyn IAdminUC> {
        Arc::new(self)
    }
}

impl IAdminUC for AdminUsecase {
    fn get_user_uc(&self) -> &dyn IUserUC {
        &*self.user_uc
    }
}
