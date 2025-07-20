use crate::entity::t_blog_user;
use async_trait::async_trait;
use common::error::BizResult;

#[async_trait]
pub trait IUserRepo: Send + Sync + shaku::Interface {
    async fn get_user(
        &self,
        user_id: Option<String>,
        username: Option<String>,
    ) -> BizResult<t_blog_user::Model>;
}
