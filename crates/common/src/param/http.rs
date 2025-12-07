use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::error::{AppErrorBuilt, AppResult};

// #[allow(dead_code)]
// pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<AppErrorBuilt>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), Json(&self)).into_response()
    }
}

impl IntoResponse for AppErrorBuilt {
    fn into_response(self) -> axum::response::Response {
        ApiResponse::<()>::err(self).into_response()
    }
}

#[allow(dead_code)]
impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok() -> Self {
        Self {
            metadata: Metadata {
                request_id: "".to_string(),
                error: None,
            },
            data: None,
        }
    }

    pub fn err(err: AppErrorBuilt) -> Self {
        Self {
            metadata: Metadata {
                request_id: "".to_string(),
                error: Some(err),
            },
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            metadata: Metadata {
                request_id: "".to_string(),
                error: None,
            },
            data: Some(data),
        }
    }

    pub fn get_status_code(&self) -> StatusCode {
        if let Some(err) = &self.metadata.error {
            let err_status_code = err.get_http_status();
            StatusCode::from_u16(*err_status_code).unwrap_or(StatusCode::BAD_REQUEST)
        } else {
            StatusCode::OK
        }
    }
}

impl<T> From<AppResult<T>> for ApiResponse<T>
where
    T: Serialize,
{
    fn from(result: AppResult<T>) -> Self {
        match result {
            Ok(data) => ApiResponse::ok_with_data(data),
            Err(err) => ApiResponse::err(err),
        }
    }
}
