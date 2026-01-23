mod mapper;

use crate::infrastructure::repository::user_repository::UserRepositoryTrait;
use infra::conn::DBManager;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(name="user_repository",  binds=[Self::into_user_repository])]
pub(crate) struct UserRepository {
    #[di(name = "pg_db")]
    pg_db: Arc<DBManager>,
}

impl UserRepository {
    fn into_user_repository(self) -> Arc<dyn UserRepositoryTrait> {
        Arc::new(self)
    }
}
