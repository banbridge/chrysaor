use axum::routing::post;
use axum::Router;

mod login;

pub fn get_routers() -> Router {
    let r: Router = Router::new().merge(login_router());

    wrap_v1(r)
}

fn login_router() -> Router {
    Router::new().route("/login", post(login::login_with_username))
}

fn wrap_v1(r: Router) -> Router {
    Router::new().nest("/api/v1", r)
}
