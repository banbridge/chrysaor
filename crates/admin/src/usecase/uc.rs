use super::user::UserUsecase;
use crate::data::AdminRepo;
use crate::domain::IUserUC;
use crate::{data, domain, util};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminUsecase {
    admin_repo: Arc<AdminRepo>,

    user_uc: Arc<dyn IUserUC>,
    data: Arc<data::Data>,

    jwt: Arc<util::jwt::JWT>,
}

impl AdminUsecase {
    pub fn new(data: Arc<data::Data>, admin_repo: Arc<AdminRepo>) -> Self {
        let config = data.config.clone();
        let jwt = Arc::new(util::jwt::JWT::new(config));

        AdminUsecase {
            admin_repo: admin_repo.clone(),

            user_uc: UserUsecase::new(admin_repo.clone(), jwt.clone()),
            data: data.clone(),
            jwt: jwt.clone(),
        }
    }
}

impl domain::IAdminUC for AdminUsecase {
    fn get_user_uc(&self) -> &dyn IUserUC {
        &*self.user_uc
    }
}
