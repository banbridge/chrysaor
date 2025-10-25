use crate::repo::mysql::factory::MySqlRepoFactory;
use crate::repo::{IRepoFactory, IUserRepo};
use std::sync::Arc;

impl IRepoFactory for MySqlRepoFactory {
    fn get_user_repo(&self) -> Arc<dyn IUserRepo> {
        self.user_repo.clone()
    }
}
