use crate::{
    entity::{prelude::TBlogUser, t_blog_user},
    repo::IUserRepo,
};
use async_trait::async_trait;
use common::error::{BizError, BizResult};
use faststr::FastStr;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(name = "user_repo", binds=[Self::into_user_repo])]
pub struct UserRepo {
    #[di(name = "mysql_db")]
    db: Arc<sea_orm::DatabaseConnection>,
}

impl UserRepo {
    fn into_user_repo(self) -> Arc<dyn IUserRepo> {
        Arc::new(self)
    }
}

#[async_trait]
impl IUserRepo for UserRepo {
    async fn get_user(
        &self,
        user_id: Option<FastStr>,
        username: Option<FastStr>,
    ) -> BizResult<t_blog_user::Model> {
        if None == user_id && None == username {
            return Err(BizError::invalid_param(FastStr::from(
                "user_id or username can not be empty",
            )));
        }

        let conditions = Condition::any()
            .add(t_blog_user::Column::UserId.eq(user_id))
            .add(t_blog_user::Column::Username.eq(username));

        let user = TBlogUser::find()
            .filter(conditions)
            .one(&*self.db)
            .await
            .map_err(|e| BizError::db_query_failed(e.to_string().into()))?
            .ok_or(BizError::invalid_param(FastStr::from(
                "user_id or username not exists",
            )))?;
        Ok(user)
    }
}
