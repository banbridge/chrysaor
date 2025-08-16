use crate::conf;
use infra::repo::IUserRepo;
use num_cpus;
use sea_orm::ConnectOptions;
use std::cmp::max;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
#[rudi::Singleton(async,name = "data", binds=[Self::into_data])]
pub struct Data {
    #[di(name = "config")]
    pub config: Arc<conf::AppConf>,
    #[di(name = "mysql_db")]
    db: Arc<sea_orm::DatabaseConnection>,
}

impl Data {
    fn into_data(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[rudi::Singleton(name = "mysql_db")]
async fn init_db(
    #[di(name = "config")] conf: Arc<conf::AppConf>,
) -> Arc<sea_orm::DatabaseConnection> {
    let mut options = ConnectOptions::new(conf.mysql_dsn());

    let cpu_cores = num_cpus::get() as u32;

    options
        .min_connections(max(cpu_cores, 10))
        .max_connections(max(cpu_cores * 8, 100))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false);

    let db = sea_orm::Database::connect(options)
        .await
        .expect("connect mysql failed");

    db.ping().await.expect("ping mysql failed");

    tracing::info!("database connection successfully!!!");

    Arc::new(db)
}

#[derive(Clone)]
#[rudi::Singleton(async,name = "admin_repo", binds=[Self::into_admin_repo])]
pub struct AdminRepo {
    #[di(name = "data")]
    data: Arc<Data>,

    #[di(name = "user_repo")]
    pub user_repo: Arc<dyn IUserRepo>,
}

impl AdminRepo {
    fn into_admin_repo(self) -> Arc<Self> {
        Arc::new(self)
    }
}
