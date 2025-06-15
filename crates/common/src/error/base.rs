use crate::param;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizError {
    status_code: u16,
    message: String,
    biz_code: u32,
}

impl BizError {
    pub fn new(status_code: u16, message: &str, biz_code: u32) -> Self {
        BizError {
            status_code,
            message: message.to_string(),
            biz_code,
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }
}

impl IntoResponse for BizError {
    fn into_response(self) -> Response {
        let staus_code =
            StatusCode::from_u16(self.status_code()).unwrap_or(StatusCode::BAD_REQUEST);

        let resp_data: param::ApiResponse<()> = param::ApiResponse::err(self);

        (staus_code, axum::Json(resp_data)).into_response()
    }
}

impl From<BizError> for Response {
    fn from(value: BizError) -> Self {
        value.into_response()
    }
}

impl Display for BizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  status_code: {}, message: {}, biz_code: {}",
            self.status_code, self.message, self.biz_code
        )
    }
}

impl std::error::Error for BizError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_biz_error() {
        let err = BizError::new(400, "test", 1);
        println!("{}", err);
    }
}
