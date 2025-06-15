use crate::error::BizError;
use crate::param::base;
use axum;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<ApiResponse<T>, BizError>;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ResponseMetadata {
    request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<BizError>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

        let body = axum::Json(self);
        (status_code, body).into_response()
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

    pub fn ok_with_page<U>(
        pagination: &base::Pagination,
        total: u64,
        row: Vec<U>,
    ) -> ApiResponse<Page<U>> {
        ApiResponse {
            response_metadata: Self::build_metadata(None),
            data: Some(Page::from_pagination(pagination, total, row)),
        }
    }

    pub fn err(err: BizError) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(Some(err)),
            data: None,
        }
    }

    pub fn new_with_code_and_msg(status_code: u16, msg: &str, biz_code: u32) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(Some(BizError::new(
                status_code,
                msg,
                biz_code,
            ))),
            data: None,
        }
    }

    fn status_code(&self) -> StatusCode {
        let err = self.response_metadata.error.as_ref();
        if err.is_some() {
            StatusCode::from_u16(err.unwrap().status_code()).unwrap()
        } else {
            StatusCode::OK
        }
    }

    fn build_metadata(biz_error: Option<BizError>) -> ResponseMetadata {
        let log_id = "";

        ResponseMetadata {
            request_id: log_id.to_string(),
            error: biz_error,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Page<T> {
    pub total: u64,
    pub page_num: u64,
    pub page_size: u64,

    pub row: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(page_num: u64, page_size: u64, total: u64, row: Vec<T>) -> Self {
        Page {
            total,
            page_num,
            page_size,
            row,
        }
    }

    pub fn from_pagination(pagination: &base::Pagination, total: u64, row: Vec<T>) -> Self {
        Page {
            total,
            page_num: pagination.page_num,
            page_size: pagination.page_size,
            row,
        }
    }
}
