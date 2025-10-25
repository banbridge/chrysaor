use crate::repo::IUserRepo;

use std::sync::Arc;

pub trait IRepoFactory: Send + Sync {
    fn get_user_repo(&self) -> Arc<dyn IUserRepo>;
}
