use common::error::AppResult;
use faststr::FastStr;
use infra::entity::user;

#[async_trait::async_trait]
pub(crate) trait UserRepositoryTrait: Send + Sync {
    async fn get_user(&self, username: FastStr) -> AppResult<user::Model>;
}
