mod init;
mod jwt;

use anyhow::Context;
use config::FileFormat;
pub use jwt::*;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConf {
    server: ServerConf,
    database: DatabaseConf,
    jwt: JwtConf,
}

#[derive(Debug, Deserialize, Clone)]
struct ServerConf {
    port: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct DatabaseConf {
    user: String,
    password: String,
    host: String,
    port: u32,
    db: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConf {
    pub secret: String,
    pub expiration: u64, // 过期时间
                         //pub issuer: String,   // 签发人
}

impl AppConf {
    pub fn load(file_name: &str, format: FileFormat) -> anyhow::Result<Arc<Self>> {
        config::Config::builder()
            .add_source(
                config::File::with_name(file_name)
                    .format(format)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()
            .with_context(|| "load app conf failed")?
            .try_deserialize()
            .with_context(|| "failed to deserialize app conf")
    }

    pub fn server_port(&self) -> u32 {
        self.server.port.unwrap_or(8089)
    }

    pub fn mysql_dsn(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset=utf8mb4&parseTime=True&loc=Local",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.db
        )
    }

    pub fn jwt(&self) -> &JwtConf {
        &self.jwt
    }
}
