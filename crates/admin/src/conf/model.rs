use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct Server {
    pub(super) port: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct Postgres {
    pub(super) user: String,
    pub(super) password: String,
    pub(super) host: String,
    pub(super) port: u32,
    pub(super) db_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct Jwt {
    pub(super) secret: String,
    pub(super) expiration: u64, // 过期时间
    pub(super) issuer: String,  // 签发人
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub(super) env: String,
    pub(super) server: Server,
    pub(super) postgres: Postgres,
    pub(super) jwt: Jwt,
}
