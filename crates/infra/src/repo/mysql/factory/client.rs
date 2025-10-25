use crate::repo::IUserRepo;
use crate::repo::mysql::user::UserRepo;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// MySQL 仓库工厂，负责创建和管理各种 MySQL 仓库
pub struct MySqlRepoFactory {
    pub(super) db: Arc<DatabaseConnection>,
    pub(super) user_repo: Arc<dyn IUserRepo>,
}

impl MySqlRepoFactory {
    /// 创建一个新的 MySQL 仓库工厂
    ///
    /// # 参数
    /// - `db`: 数据库连接，将被包装在 Arc 中以支持共享所有权
    ///
    /// # 返回值
    /// 返回一个新的 `MySqlRepoFactory` 实例
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        let user_repo = UserRepo::new(db.clone());

        Self {
            db,
            user_repo: Arc::new(user_repo),
        }
    }

    pub fn get_db(&self) -> Arc<DatabaseConnection> {
        self.db.clone()
    }
}
