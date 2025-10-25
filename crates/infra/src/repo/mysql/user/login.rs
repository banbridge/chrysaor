use crate::entity::prelude::TBlogUser;
use crate::entity::t_blog_user;
use crate::repo::IUserRepo;
use crate::repo::mysql::user::UserRepo;
use async_trait::async_trait;
use common::{
    check::require_non_empty,
    error::{BizError, BizResult},
};
use faststr::FastStr;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

#[async_trait]
impl IUserRepo for UserRepo {
    async fn get_user(&self, username_or_id: FastStr) -> BizResult<t_blog_user::Model> {
        require_non_empty(&username_or_id, "UsernameOrId can not be empty")?;

        let conditions = Condition::any()
            .add(t_blog_user::Column::UserId.eq(username_or_id.clone()))
            .add(t_blog_user::Column::Username.eq(username_or_id));

        let user = TBlogUser::find()
            .filter(conditions)
            .one(self.db.as_ref())
            .await
            .map_err(|e| BizError::db_query_failed(e.to_string().into()))?
            .ok_or(BizError::invalid_param(FastStr::from(
                "user_id or username not exists",
            )))?;
        Ok(user)
    }
}
