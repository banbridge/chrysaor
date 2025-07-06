use crate::conf;
use infra::repo;
use infra::repo::IUserRepo;
use num_cpus;
use sea_orm::ConnectOptions;
use std::cmp::max;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct Data {
    pub(crate) config: Arc<conf::AppConf>,
    db: Arc<sea_orm::DatabaseConnection>,
}

impl Data {
    pub async fn new(config: Arc<conf::AppConf>) -> anyhow::Result<Arc<Self>> {
        let db_result = Self::init_db(Arc::clone(&config)).await?;
        let res: Data = Data {
            config: Arc::clone(&config),
            db: db_result,
        };
        Ok(Arc::new(res))
    }

    async fn init_db(conf: Arc<conf::AppConf>) -> anyhow::Result<Arc<sea_orm::DatabaseConnection>> {
        let mut options = ConnectOptions::new(conf.mysql_dsn());

        let cpu_cores = num_cpus::get() as u32;

        options
            .min_connections(max(cpu_cores, 10))
            .max_connections(max(cpu_cores * 8, 20))
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(3600 * 24))
            .sqlx_logging(false);

        let db = sea_orm::Database::connect(options).await?;

        db.ping().await?;

        tracing::info!("database connection successfully!!!");

        Ok(Arc::new(db))
    }
}

pub struct AdminRepo {
    data: Arc<Data>,

    user_repo: Arc<dyn IUserRepo>,
}

impl AdminRepo {
    pub fn new(data: Arc<Data>) -> Arc<Self> {
        let user_repo = Arc::new(repo::mysql::UserRepo::new(data.db.clone()));
        Arc::new(AdminRepo { data, user_repo })
    }

    pub fn user_repo(&self) -> Arc<dyn IUserRepo> {
        Arc::clone(&self.user_repo)
    }
}
