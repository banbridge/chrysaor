use axum::Router;

pub(crate) mod login;

pub fn get_routers() -> Router {
    Router::new().merge(login::get_routers())
}
