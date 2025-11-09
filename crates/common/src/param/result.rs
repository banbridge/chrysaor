use crate::error::AppErrorBizBuilder;
use crate::param::ApiResponse;

pub type ApiResult<T> = Result<ApiResponse<T>, AppErrorBizBuilder>;
