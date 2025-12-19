use super::GLOBAL_JWT;
use crate::app::AdminService;
use axum::{extract::FromRequestParts, http::request::Parts, routing::get, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use common::error::{AppErrorBuilt, AppResult};
use faststr::FastStr;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters)]
#[serde(rename_all = "PascalCase")]
#[getset(get = "pub", set = "pub")]
pub struct User {
    user_id: FastStr,
    username: FastStr,
}

impl User {
    pub fn new(user_id: FastStr, username: FastStr) -> Self {
        User { user_id, username }
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = AppErrorBuilt;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Self> {
        let admin_svc = parts
            .extensions
            .get::<AdminService>()
            .ok_or(AppErrorBuilt::invalid_param("service not init".to_string()))?;

        // todo 权限校验

        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| AppErrorBuilt::jwt_invalid_token(format!("extract jwt failed {}", e)))?;

        // Decode the user data
        let token_data = admin_svc.jwt_manager.decode::<User>(bearer.token())?;

        Ok(token_data)
    }
}
