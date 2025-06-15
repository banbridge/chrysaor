use admin::service::service::*;
use admin::usercase::uc::*;
use axum::{Router, routing::post};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = new_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8099").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn new_app() -> Router {
    let uc = AdminUsercase::new();
    let svc = AdminService::new(Box::new(uc));
    let app = Router::new().route("/hello", post(svc.say_hello));
    app
}
