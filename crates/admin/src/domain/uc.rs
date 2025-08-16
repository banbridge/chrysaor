use async_trait::async_trait;
use faststr::FastStr;
use common::error::BizResult;
use crate::usecase;

#[async_trait]
pub trait IAdminUC: Send + Sync {
    fn get_user_uc(&self) -> &dyn IUserUC;
}

#[async_trait]
pub trait IUserUC: Send + Sync {
    async fn list_user(&self, req: usecase::ListUserInput) -> BizResult<usecase::ListUserOutput>;
    async fn login(&self, req: usecase::LoginInput) -> BizResult<FastStr>;
}
