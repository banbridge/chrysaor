use super::user::UserUsecase;
use crate::domain::IUserUC;
use crate::{data, domain, util};
use std::sync::Arc;
use crate::data::AdminRepo;

#[derive(Clone)]
pub struct AdminUsecase {
    admin_repo: Arc<AdminRepo>,
    
    user_uc: Arc<dyn IUserUC>,
    data: Arc<data::Data>,

    jwt: Arc<util::jwt::JWT>
}


impl AdminUsecase {
    pub fn new(data: Arc<data::Data>, admin_repo: Arc<AdminRepo>) -> Self {
        let config = Arc::clone(&data.config);
        let jwt = Arc::new(util::jwt::JWT::new(config));

        AdminUsecase {
            admin_repo: Arc::clone(&admin_repo),
            
            user_uc: UserUsecase::new(Arc::clone(&admin_repo), Arc::clone(&jwt)),
            data: Arc::clone(&data),
            jwt,
        }
    }
}

impl domain::IAdminUC for AdminUsecase {
    fn get_user_uc(&self) -> &dyn IUserUC {
        &*self.user_uc 
    }
}
