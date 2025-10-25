use crate::error::BizError;
use crate::param::ApiResponse;

pub type ApiResult<T> = Result<ApiResponse<T>, BizError>;
