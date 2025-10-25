use crate::helper;
use crate::service::AdminService;
use common::middle;
use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};
use volo::net::Address;
use volo_http::server::{Router, Server, layer::TimeoutLayer, middleware};
use volo_http::utils::Extension;

#[rudi::Singleton]
pub async fn run(#[di(name = "admin_service")] admin_service: Arc<AdminService>) {
    //println!("{:?}", config);
    let app = Router::new()
        .merge(wrap_router_v1(
            "/user",
            admin_service.login_controller.get_routers_v1(),
        ))
        .layer(TimeoutLayer::new(
            Duration::from_secs(5),
            middle::timeout_middleware,
        ))
        .layer(middleware::from_fn(helper::jwt_auth_middleware))
        .layer(middleware::from_fn(middle::request_timer_middleware))
        .layer(middleware::from_fn(middle::log_id_middleware))
        .layer(Extension(admin_service.clone()));

    let port = admin_service.config.server_port();
    let addr_str = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    let addr = Address::from(addr_str);

    Server::new(app).run(addr).await.unwrap();
}

fn wrap_router_v1(prefix: &str, router: Router) -> Router {
    let v1_router = format!("/api/v1{}", prefix);
    Router::new().nest(
        v1_router,
        router.fallback(middle::failed_fallback_middleware),
    )
}
