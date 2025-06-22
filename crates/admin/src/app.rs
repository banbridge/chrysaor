use crate::service::service::AdminService;
use crate::usecase::uc::AdminUsecase;
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

    //println!("{:?}", config);

    init_logger(&config);

    let admin_service = wire_gen(Arc::clone(&config)).await?;

    let server = server::Server::new(&config);

    let router = service::create_router();

    server.start(admin_service, router).await?;

    Ok(())
}

async fn wire_gen(config: Arc<conf::AppConf>) -> anyhow::Result<AdminService> {
    let data = data::Data::new(Arc::clone(&config)).await?;

    let admin_repo = data::AdminRepo::new(Arc::clone(&data));

    let admin_uc = Arc::new(AdminUsecase::new(
        Arc::clone(&data),
        Arc::clone(&admin_repo),
    ));

    let admin_service = AdminService::new(Arc::clone(&config), admin_uc);
    Ok(admin_service)
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
