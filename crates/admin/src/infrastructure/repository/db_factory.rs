use super::user_repository::UserRepositoryTrait;
use infra::conn::DBManager;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait DBFactoryTrait: Send + Sync {
    fn get_db_manager(&self) -> Arc<DBManager>;

    fn get_user_repository(&self) -> Arc<dyn UserRepositoryTrait>;
}
