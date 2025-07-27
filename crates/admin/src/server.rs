use crate::service::service::AdminService;
use crate::{conf, util};

use anyhow::Ok;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::Response;
use axum::{Router, middleware};
use bytesize::ByteSize;

use common::context;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::trace::{OnResponse, TraceLayer};
use tracing::Span;

pub struct Server {
    config: Arc<conf::AppConf>,
}

impl Server {
    pub fn new(conf: Arc<conf::AppConf>) -> Self {
        Server { config: conf }
    }

    pub async fn start(
        &self,
        admin_service: AdminService,
        router: Router<AdminService>,
    ) -> anyhow::Result<()> {
        let router = self.build_router(admin_service, router);

        let port = self.config.server_port();
        let addr = format!("0.0.0.0:{}", port);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(addr).await?;

        tracing::info!("listening on {}", listener.local_addr()?);

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, admin_service: AdminService, router: Router<AdminService>) -> Router {
        let tracing_l = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method().to_string();
                let path = request.uri().path().to_string();
                let log_id = context::get_or_default_log_id();

                tracing::info_span!("api request", path = %path, method = %method, log_id = %log_id)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyResponse);

        let timeout_l = tower_http::timeout::TimeoutLayer::new(Duration::from_secs(30));
        let body_limit_l = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        let cors_l = tower_http::cors::CorsLayer::new()
            .allow_methods(tower_http::cors::Any)
            .allow_origin(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));
        let normalize_path_l =
            tower_http::normalize_path::NormalizePathLayer::trim_trailing_slash();

        Router::new()
            .merge(router)
            // .layer(middleware::from_fn_with_state(
            //     admin_service.clone(),
            //     crate::middleware::jwt_auth,
            // ))
            .layer(timeout_l)
            .layer(body_limit_l)
            .layer(cors_l)
            .layer(middleware::from_fn(crate::middleware::request_middleware))
            .layer(tracing_l)
            .layer(middleware::from_fn(crate::middleware::request_timer))
            .layer(normalize_path_l)
            .with_state(admin_service)
    }
}

#[derive(Clone, Debug, Copy)]
struct LatencyResponse;

impl<B> OnResponse<B> for LatencyResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = response.status().as_u16(),
            "finished processing request"
        );
    }
}

struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ms = self.0.as_millis();
        if ms > 0 {
            write!(f, "{} ms", ms)
        } else {
            write!(f, "{} us", self.0.as_micros())
        }
    }
}
