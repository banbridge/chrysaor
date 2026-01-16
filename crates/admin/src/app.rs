use std::sync::Arc;

use crate::biz::login::LoginCommandService;
use crate::conf::AdminConfig;
use common::context::init_logger;
use infra::casbin_auth::CasbinManager;

#[derive(Clone)]
#[rudi::Singleton(async, name = "admin_service", binds=[Self::into_admin_service])]
pub struct AdminService {
    #[di(name = "config")]
    pub config: Arc<AdminConfig>,
    // #[di(name = "login_controller")]
    // pub login_controller: Arc<LoginController>,
    #[di(name = "pg_db")]
    pub pg_db: Arc<sea_orm::DatabaseConnection>,

    #[di(name = "jwt_manager")]
    pub jwt_manager: Arc<common::jwt::JWTManager>,

    #[di(name = "casbin_manager")]
    pub casbin_manager: Arc<CasbinManager>,

    #[di(name = "login_command_service")]
    pub login_command_service: Arc<LoginCommandService>,
}

impl AdminService {
    fn into_admin_service(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[rudi::Singleton(name = "config")]
pub fn get_config() -> Arc<AdminConfig> {
    dotenv::dotenv().ok();

    let conf_info = AdminConfig::load().unwrap();

    init_logger();

    Arc::new(conf_info)
}

#[rudi::Singleton(name = "pg_db")]
async fn init_db(
    #[di(name = "config")] conf: Arc<AdminConfig>,
) -> Arc<sea_orm::DatabaseConnection> {
    let db = infra::conn::get_postgresql_db(conf.get_db_dsn().as_str())
        .await
        .unwrap();

    Arc::new(db)
}

#[rudi::Singleton(name = "jwt_manager")]
async fn init_jwt_manager(
    #[di(name = "config")] conf: Arc<AdminConfig>,
) -> Arc<common::jwt::JWTManager> {
    Arc::new(conf.get_jwt())
}

#[rudi::Singleton(name = "casbin_manager")]
async fn init_casbin_manager(
    #[di(name = "pg_db")] db: Arc<sea_orm::DatabaseConnection>,
) -> Arc<CasbinManager> {
    let res = CasbinManager::new(db.as_ref().clone())
        .await
        .expect("init casbin manager failed");
    Arc::new(res)
}
