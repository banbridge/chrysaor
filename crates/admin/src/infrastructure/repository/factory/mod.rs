mod mapper;

use super::{DBFactoryTrait, UserRepositoryTrait};
use infra::conn::DBManager;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(name="db_factory", binds=[Self::into_db_factory])]
pub struct DBFactory {
    #[di(name = "db_manager")]
    db_manager: Arc<DBManager>,

    #[di(name = "user_repository")]
    user_repo: Arc<dyn UserRepositoryTrait>,
}

impl DBFactory {
    fn into_db_factory(self) -> Arc<dyn DBFactoryTrait> {
        Arc::new(self)
    }
}
