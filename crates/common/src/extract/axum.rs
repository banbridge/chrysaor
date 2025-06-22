use crate::error::BizError;
use crate::param::ApiResponse;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Clone)]
pub struct BindAndValidate<T>(pub T);

impl<T, S> FromRequest<S> for BindAndValidate<T>
where
    T: DeserializeOwned + ::prost_validate::Validator,
    S: Send + Sync,
    axum::Json<T>: FromRequest<S, Rejection = axum::extract::rejection::JsonRejection>,
{
    type Rejection = ApiResponse<()>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 从请求中提取JSON数据
        let axum::Json(data) = axum::Json::from_request(req, state)
            .await
            .map_err(|err| ApiResponse::err(BizError::param_bind(err.to_string().as_str())))?;

        // 验证数据
        data.validate()
            .map_err(|err| ApiResponse::err(BizError::invalid_param(err.to_string().as_str())))?;

        Ok(Self(data))
    }
}
