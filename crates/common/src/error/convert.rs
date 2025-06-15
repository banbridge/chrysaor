use crate::error::BizError;
use jsonwebtoken::errors::ErrorKind;

impl From<jsonwebtoken::errors::Error> for BizError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.kind() {
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidEcdsaKey
            | ErrorKind::ExpiredSignature
            | ErrorKind::RsaFailedSigning
            | ErrorKind::MissingAlgorithm
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::InvalidKeyFormat
            | ErrorKind::InvalidAlgorithmName => {
                BizError::jwt_invalid_token(value.to_string().as_str())
            }
            ErrorKind::MissingRequiredClaim(c) => {
                BizError::jwt_invalid_token(format!("Missing required claim: {}", c).as_str())
            }
            ErrorKind::InvalidRsaKey(msg) => {
                BizError::jwt_invalid_token(format!("RSA key invalid: {}", msg).as_str())
            }
            ErrorKind::Json(err) => {
                BizError::jwt_invalid_token(format!("JSON error: {}", err).as_str())
            }
            ErrorKind::Utf8(err) => {
                BizError::jwt_invalid_token(format!("UTF-8 error: {}", err).as_str())
            }
            ErrorKind::Crypto(err) => {
                BizError::jwt_invalid_token(format!("Crypto error: {}", err).as_str())
            }
            ErrorKind::Base64(err) => {
                BizError::jwt_invalid_token(format!("Base64 error: {}", err).as_str())
            }
            &_ => BizError::jwt_invalid_token(value.to_string().as_str()),
        }
    }
}

impl From<serde_json::error::Error> for BizError {
    fn from(value: serde_json::error::Error) -> Self {
        BizError::json_serde(
            format!("JSON serde code({:?}) error: {}", value.classify(), value).as_str(),
        )
    }
}

impl From<anyhow::Error> for BizError {
    fn from(value: anyhow::Error) -> Self {
        if let Some(biz_err) = value.downcast_ref::<BizError>() {
            biz_err.to_owned()
        } else {
            BizError::unknown_anyhow(value)
        }
    }
}
