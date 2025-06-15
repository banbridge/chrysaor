use super::user::UserUsecase;
use crate::domain::IUserUC;
use crate::{data, domain};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminUsecase {
    user_uc: Arc<dyn IUserUC>,
    data: data::Data,
}

impl AdminUsecase {
    pub fn new(data: &data::Data) -> Self {
        AdminUsecase {
            user_uc: Arc::new(UserUsecase {}),
            data: data.clone(),
        }
    }
}

impl domain::IAdminUC for AdminUsecase {
    fn get_user_uc(&self) -> &dyn IUserUC {
        &*self.user_uc
    }
}
