#[allow(unused_imports)]
use admin::app::AdminService;
use admin::router::get_app_routers;
use axum::Extension;
use common::{middle, util};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    let mut ctx = rudi::Context::auto_register();
    ctx.resolve_async::<()>().await
}

#[rudi::Singleton]
pub async fn run(#[di(name = "admin_service")] admin_service: Arc<AdminService>) {
    //println!("{:?}", config);

    let router = get_app_routers();

    let router = router.fallback(middle::handler_404);

    let app = middle::add_middleware_list(router).layer(Extension(admin_service.clone()));

    let port = admin_service.config.get_server_port();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:}", port))
        .await
        .unwrap();

    info!("admin service start at: {}", port);

    axum::serve(listener, app)
        .with_graceful_shutdown(util::shutdown_signal())
        .await
        .unwrap();
}
