use sea_orm::ConnectOptions;
use std::cmp::max;
use std::time::Duration;

pub mod factory;
pub mod user;

pub async fn get_db(dsn:String)->sea_orm::DatabaseConnection{

    let mut options = ConnectOptions::new(dsn);

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

    db
}