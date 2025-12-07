use crate::{
    error::{AppErrorBuilt, AppResult},
    jwt::{Claim, Principal},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use std::time::Duration;

#[derive(Clone)]
pub struct JWT {
    encode_key: EncodingKey,
    decode_key: DecodingKey,

    header: Header,
    validation: Validation,
    expiration: Duration,
}

// impl Display for JWT {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "JWT   header: {:?}, validation: {:?}, expiration: {:?} ",
//             self.header,
//             self.validation,
//             self.expiration.as_secs(),
//         )
//     }
// }

impl JWT {
    pub fn new(encode_key: &str, expiration_second: u64) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["me"]);
        validation.set_required_spec_claims(&["exp", "sub", "iat"]);

        Self {
            encode_key: EncodingKey::from_secret(encode_key.as_bytes()),
            decode_key: DecodingKey::from_secret(encode_key.as_bytes()),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: Duration::from_secs(expiration_second),
        }
    }

    pub fn with_expiration(mut self, expiration_second: u64) -> Self {
        self.expiration = Duration::from_secs(expiration_second);
        self
    }

    pub fn encode(&self, principal: Principal) -> AppResult<String> {
        let current_time = get_current_timestamp();

        let claim = Claim {
            sub: principal.user_id,
            exp: current_time.saturating_add(self.expiration.as_secs()),
            iat: current_time,

            username: principal.username,
        };

        let jwt_token =
            jsonwebtoken::encode(&self.header, &claim, &self.encode_key).map_err(|err| {
                AppErrorBuilt::jwt_encode(format!("jwt encode err: {:?}", err).into())
            })?;

        Ok(jwt_token.into())
    }

    pub fn decode(&self, token: &str) -> AppResult<Principal> {
        let token_data = jsonwebtoken::decode::<Claim>(token, &self.decode_key, &self.validation)
            .map_err(|err| {
            AppErrorBuilt::jwt_decode(format!("jwt decode err: {:?}", err).into())
        })?;

        Ok(token_data.claims.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct Claims {
        aud: String,
        sub: Principal,
        company: String,
        exp: u64,
    }

    #[test]
    fn test_jwt() {
        let jwt = JWT::new("secret", 3600);

        let p = Principal {
            user_id: "b@b.com".to_string(),
            username: "b@b.com".to_string(),
        };
        let token = jwt.encode(p).unwrap();
        println!("{}", token);

        let p = jwt.decode(&token);
        println!("{:?}", p);
    }
}
