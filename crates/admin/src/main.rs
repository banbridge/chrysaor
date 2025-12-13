use axum::{Router, routing::get};
use common::{context, middle, param::ApiResponse};
use tokio::signal;

#[tokio::main]
async fn main() {
    context::init_logger();
    let router = Router::new().route("/hello", get(hello_handler));

    let app = middle::add_middleware_list(router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn hello_handler() -> ApiResponse<()> {
    ApiResponse::ok()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
