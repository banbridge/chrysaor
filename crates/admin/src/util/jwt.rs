use crate::conf;
use common::error::BizError;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use faststr::FastStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Principal {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claim {
    pub jti: String,    // jwt id
    pub sub: Principal, // 面向的用户
    pub aud: String,    // 接收jwt的一方
    pub exp: u64,       // 过期时间
    pub iat: u64,       // 签发时间
    pub iss: String,    // 签发人
}

#[derive(Clone)]
pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,

    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

#[rudi::Singleton(name = "jwt", binds = [Self::into_jwt])]
impl JWT {
    #[di]
    pub fn new(#[di(name = "config")] config: Arc<conf::AppConf>) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.jwt().audience]);
        validation.set_issuer(&[&config.jwt().issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "exp", "iat", "iss"]);

        JWT {
            encode_secret: EncodingKey::from_secret(config.jwt().secret.as_ref()),
            decode_secret: DecodingKey::from_secret(config.jwt().secret.as_ref()),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: Duration::from_secs(config.jwt().expiration),
            audience: config.jwt().audience.clone(),
            issuer: config.jwt().issuer.clone(),
        }
    }

    fn into_jwt(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn encode(&self, principal: &Principal) -> anyhow::Result<FastStr> {
        let current_time = get_current_timestamp();

        let claim = Claim {
            jti: xid::new().to_string(),
            sub: principal.clone(),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_time,
            exp: current_time.saturating_add(self.expiration.as_secs()),
        };

        let jwt_token = jsonwebtoken::encode(&self.header, &claim, &self.encode_secret)
            .map_err(|err| BizError::jwt_encode(err.to_string().into()))?;

        Ok(jwt_token.into())
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let decoded_claim: Claim =
            jsonwebtoken::decode::<Claim>(token, &self.decode_secret, &self.validation)
                .map_err(|err| BizError::jwt_decode(err.to_string().into()))?
                .claims;

        let principal = decoded_claim.sub;
        Ok(principal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conf::AppConf;
    #[test]
    fn test_jwt() {}
}
