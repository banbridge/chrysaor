use crate::conf;
use num_cpus;
use sea_orm::ConnectOptions;
use std::cmp::max;
use std::time::Duration;

#[derive(Clone)]
pub struct Data {
    config: conf::AppConf,
    pub db: sea_orm::DatabaseConnection,
}

impl Data {
    pub async fn new(config: &conf::AppConf) -> anyhow::Result<Self> {
        let config_copy = config.clone();
        let db_result = Self::init_db(config).await?;
        let res: Data = Data {
            config: config_copy,
            db: db_result,
        };
        Ok(res)
    }

    async fn init_db(conf: &conf::AppConf) -> anyhow::Result<sea_orm::DatabaseConnection> {
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

        Ok(db)
    }
}
