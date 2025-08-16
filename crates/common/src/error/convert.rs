use crate::error::BizError;

impl From<serde_json::error::Error> for BizError {
    fn from(value: serde_json::error::Error) -> Self {
        BizError::json_serde(
            format!("JSON serde code({:?}) error: {}", value.classify(), value).into(),
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
