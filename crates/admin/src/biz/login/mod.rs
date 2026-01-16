use axum::routing::post;
use axum::Router;

mod command;
mod controller;

pub use command::LoginCommandService;

pub fn get_routers() -> Router {
    Router::new().nest(
        "/user",
        Router::new().route("/login", post(controller::login)),
    )
}
