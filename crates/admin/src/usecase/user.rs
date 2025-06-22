use crate::data::AdminRepo;
use crate::domain;
use async_trait::async_trait;
use std::sync::Arc;
use common::error::BizResult;
use super::{ListUserInput, ListUserOutput};

pub struct UserUsecase {
    admin_repo: Arc<AdminRepo>,
}

impl UserUsecase {
    pub fn new(admin_repo: Arc<AdminRepo>) -> Arc<Self> {
        Arc::new(UserUsecase { admin_repo })
    }
}

#[async_trait]
impl domain::IUserUC for UserUsecase {
    async fn list_user(&self, _req: ListUserInput) -> BizResult<ListUserOutput> {
        let output  = ListUserOutput {
            total: 0,
            list: vec![],
        };
        
        Ok(output)
    }
}
