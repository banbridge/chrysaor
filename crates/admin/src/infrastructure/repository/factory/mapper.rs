use super::{DBFactory, DBFactoryTrait};
use crate::infrastructure::repository::UserRepositoryTrait;
use infra::conn::DBManager;
use std::sync::Arc;

#[async_trait::async_trait]
impl DBFactoryTrait for DBFactory {
    fn get_db_manager(&self) -> Arc<DBManager> {
        self.db_manager.clone()
    }

    fn get_user_repository(&self) -> Arc<dyn UserRepositoryTrait> {
        self.user_repo.clone()
    }
}
