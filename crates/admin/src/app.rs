use crate::service::service::AdminService;
use crate::usercase::uc::AdminUsecase;
use crate::{conf, data, server, service};
use axum::debug_handler;
use axum::extract::State;
use common::param::{ApiResponse, ApiResult};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn run() -> anyhow::Result<()> {
    let config = conf::AppConf::load("config/admin/app", config::FileFormat::Yaml)?;

    println!("{:?}", config);

    init_logger(&config);

    let data = data::Data::new(&config).await?;

    let admin_uc = Arc::new(AdminUsecase::new(&data));

    let admin_service = AdminService::new(&config, admin_uc);

    let server = server::Server::new(&config);

    let router = service::create_router();

    server.start(admin_service, router).await?;

    Ok(())
}

pub fn init_logger(conf: &conf::AppConf) {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_target(false),
        )
        .init()
}
