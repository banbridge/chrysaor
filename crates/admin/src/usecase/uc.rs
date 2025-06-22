use super::user::UserUsecase;
use crate::domain::IUserUC;
use crate::{data, domain};
use std::sync::Arc;
use async_trait::async_trait;
use crate::data::AdminRepo;

#[derive(Clone)]
pub struct AdminUsecase {
    admin_repo: Arc<AdminRepo>,
    
    user_uc: Arc<dyn IUserUC>,
    data: Arc<data::Data>,
}


impl AdminUsecase {
    pub fn new(data: Arc<data::Data>, admin_repo: Arc<AdminRepo>) -> Self {
        AdminUsecase {
            admin_repo: Arc::clone(&admin_repo),
            
            user_uc: UserUsecase::new(Arc::clone(&admin_repo)),
            data: Arc::clone(&data),
        }
    }
}

impl domain::IAdminUC for AdminUsecase {
    fn get_user_uc(&self) -> &dyn IUserUC {
        &*self.user_uc
    }
}
