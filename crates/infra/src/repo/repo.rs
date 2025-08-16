use crate::entity::t_blog_user;
use async_trait::async_trait;
use common::error::BizResult;
use faststr::FastStr;

#[async_trait]
pub trait IUserRepo: Send + Sync {
    async fn get_user(
        &self,
        user_id: Option<FastStr>,
        username: Option<FastStr>,
    ) -> BizResult<t_blog_user::Model>;
}
