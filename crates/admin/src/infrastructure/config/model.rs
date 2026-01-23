use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Server {
    pub(crate) port: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Postgres {
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) host: String,
    pub(crate) port: u32,
    pub(crate) db_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Jwt {
    pub(crate) secret: String,
    pub(crate) expiration: u64, // 过期时间
    pub(crate) issuer: String,  // 签发人
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub(crate) env: String,
    pub(crate) server: Server,
    pub(crate) postgres: Postgres,
    pub(crate) jwt: Jwt,
}
