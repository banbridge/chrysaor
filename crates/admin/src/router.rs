use crate::biz;
use axum::Router;

pub fn get_app_routers() -> Router {
    wrap_v1(biz::get_routers())
}

fn wrap_v1(r: Router) -> Router {
    Router::new().nest("/api/v1", r)
}
