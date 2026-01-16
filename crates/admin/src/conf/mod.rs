mod model;

pub use model::Config as AdminConfig;

use common::{error::AppResult, jwt, util};
impl AdminConfig {
    pub fn load() -> AppResult<Self> {
        util::load_config::<AdminConfig>("config/admin/app.yaml", config::FileFormat::Yaml)
    }

    pub fn get_db_dsn(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres.user,
            self.postgres.password,
            self.postgres.host,
            self.postgres.port,
            self.postgres.db_name,
        )
    }

    pub fn get_jwt(&self) -> jwt::JWTManager {
        jwt::JWTManager::new(
            &self.jwt.secret.as_str(),
            self.jwt.expiration,
            &self.jwt.issuer.as_str(),
        )
    }

    pub fn get_server_port(&self) -> u32 {
        self.server.port.unwrap_or(8080)
    }
}
