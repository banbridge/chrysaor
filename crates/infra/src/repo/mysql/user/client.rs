use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepo {
    pub(crate) db: Arc<sea_orm::DatabaseConnection>,
}

impl UserRepo {
    pub fn new(db: Arc<sea_orm::DatabaseConnection>) -> Self {
        Self { db }
    }
}
