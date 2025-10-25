use crate::error::BizError;
use crate::param::ApiResponse;
use serde::de::DeserializeOwned;
use validator::Validate;
use volo_http::context::ServerContext;
use volo_http::error::ExtractBodyError;
use volo_http::http::request::Parts;
use volo_http::hyper::body::Body;
use volo_http::server::extract::{FromRequest, Json};

#[derive(Clone)]
pub struct BindAndValidate<T>(pub T);

impl<B, T> FromRequest<B> for BindAndValidate<T>
where
    T: DeserializeOwned + Validate,
    B: Body + Send + Sync,
    Json<T>: FromRequest<B, Rejection = ExtractBodyError>,
{
    type Rejection = ApiResponse<()>;

    async fn from_request(
        cx: &mut ServerContext,
        parts: Parts,
        body: B,
    ) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::from_request(cx, parts, body)
            .await
            .map_err(|err| ApiResponse::err(BizError::param_bind(err.to_string().into())))?;

        data.validate()
            .map_err(|err| ApiResponse::err(BizError::invalid_param(err.to_string().into())))?;
        Ok(Self(data))
    }
}
