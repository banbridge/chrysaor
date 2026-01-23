use super::UserRepository;
use crate::infrastructure::repository::user_repository::UserRepositoryTrait;
use async_trait::async_trait;
use common::error::AppResult;
use faststr::FastStr;
use infra::dbop;
use infra::entity::user::{Column, Entity, Model};
use sea_orm::{ColumnTrait, Condition};

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn get_user(&self, user_id: FastStr) -> AppResult<Model> {
        let condition = Condition::all().add(Column::Username.eq(user_id.as_str()));

        dbop::get_by_cond::<Entity>(self.pg_db.get_db(), &condition).await
    }
}
