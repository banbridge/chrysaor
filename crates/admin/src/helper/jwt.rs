use common::error::{BizError, BizResult};
use common::jwt::{JWT, Principal};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use volo::context::Context;
use volo::net::Address;
use volo_http::{
    body::Body,
    context::ServerContext,
    http::{Uri, header::AUTHORIZATION},
    request::Request,
    server::{IntoResponse, middleware::Next},
};

// 定义全局变量：用 Mutex 包裹，支持多线程安全修改
lazy_static! {
    pub static ref GLOBAL_JWT: RwLock<Arc<JWT>> =
        RwLock::new(Arc::new(JWT::new("xmcnbvjslsdjk", 100000)));
}

pub async fn jwt_auth_middleware(
    _peer: Address,
    _uri: Uri,
    cx: &mut ServerContext,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let token = match req.headers().get(AUTHORIZATION) {
        Some(token) => token.to_str().unwrap_or_default(),
        None => return BizError::unauthenticated("token is empty".into()).into_response(),
    };

    let principal: BizResult<Principal>;

    {
        let jwt = GLOBAL_JWT.read().map_err(|e| {
            BizError::internal(format!("read global jwt error: {}", e.to_string()).into())
        });
        match jwt {
            Ok(jwt) => {
                principal = jwt.decode(token);
            }
            Err(e) => return e.into_response(),
        }
    }

    tracing::info!("token: {} \n, user: {:?}", token, principal);

    match principal {
        Ok(principal) => {
            cx.extensions_mut().insert(principal);
            next.run(cx, req).await.into_response()
        }
        Err(e) => e.into_response(),
    }
}
