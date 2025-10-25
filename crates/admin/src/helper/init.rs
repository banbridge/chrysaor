use crate::helper::{AppConf, GLOBAL_JWT};
use common::jwt::JWT;
use common::log::init_logger;
use infra::repo;
use infra::repo::mysql::{factory, get_db};
use std::sync::Arc;

#[rudi::Singleton(name = "config")]
pub fn get_config() -> Arc<AppConf> {
    let conf_info = AppConf::load("config/admin/app", config::FileFormat::Yaml).unwrap();

    init_logger();

    conf_info
}

#[rudi::Singleton(name = "mysql_db")]
async fn init_db(#[di(name = "config")] conf: Arc<AppConf>) -> Arc<sea_orm::DatabaseConnection> {
    let db = get_db(conf.mysql_dsn()).await;

    Arc::new(db)
}

#[rudi::Singleton(name = "jwt")]
async fn init_jwt(#[di(name = "config")] conf: Arc<AppConf>) -> Arc<JWT> {
    let jwt = JWT::new(conf.jwt.secret.as_str(), conf.jwt.expiration);

    let res = Arc::new(jwt);

    {
        if let Ok(mut token) = GLOBAL_JWT.write() {
            *token = res.clone();
        }
    }
    res
}

#[rudi::Singleton(name = "repo_factory")]
async fn get_repo_factory(
    #[di(name = "mysql_db")] db: Arc<sea_orm::DatabaseConnection>,
) -> Arc<dyn repo::IRepoFactory> {
    Arc::new(factory::MySqlRepoFactory::new(db.clone()))
}
