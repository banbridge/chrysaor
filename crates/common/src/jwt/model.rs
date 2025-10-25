use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Principal {
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claim {
    //pub jti: String,    // jwt id
    pub sub: String, // 面向的用户
    pub exp: u64,    // 过期时间
    pub iat: u64,    // 签发时间

    pub username: String, // 用户名
}

impl From<Claim> for Principal {
    fn from(claim: Claim) -> Self {
        Principal {
            user_id: claim.sub,
            username: claim.username,
        }
    }
}
