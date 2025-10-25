use crate::context;
use crate::error::BizError;
use faststr::FastStr;
use serde::{Deserialize, Serialize};
use volo_http::server::extract::Json;
use volo_http::{http::StatusCode, response::Response, server::IntoResponse};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
struct ResponseMetadata {
    request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<BizError>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse<T> {
    response_metadata: ResponseMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        let body = Json(self);
        (status_code, body).into_response()
    }
}

impl IntoResponse for BizError {
    fn into_response(self) -> Response {
        ApiResponse::<()>::err(self).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    pub fn ok() -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(None),
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(None),
            data: Some(data),
        }
    }

    pub fn err(err: BizError) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(Some(err)),
            data: None,
        }
    }

    pub fn new_with_code_and_msg(status_code: u16, msg: FastStr, biz_code: u32) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(Some(BizError::new(
                status_code,
                msg,
                biz_code,
                FastStr::from("Unknown"),
            ))),
            data: None,
        }
    }

    fn status_code(&self) -> StatusCode {
        let err = self.response_metadata.error.as_ref();
        if let Some(err) = err {
            StatusCode::from_u16(err.status_code()).unwrap()
        } else {
            StatusCode::OK
        }
    }

    fn build_metadata(biz_error: Option<BizError>) -> ResponseMetadata {
        // let log_id = logid::get_or_defalue_logid();
        let log_id = context::get_or_default_log_id();
        ResponseMetadata {
            request_id: log_id.to_string(),
            error: biz_error,
        }
    }
}
