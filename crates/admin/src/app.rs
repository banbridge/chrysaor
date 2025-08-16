use crate::service::service::AdminService;
use crate::{conf, server, service};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[rudi::Singleton]
pub async fn run(#[di(name = "admin_service")] admin_service: AdminService) {
    //println!("{:?}", config);

    let server = server::Server::new(admin_service.clone().config);

    let router = service::create_router();

    server.start(admin_service, router).await.unwrap();
}

#[rudi::Singleton(name = "config")]
pub fn get_config() -> Arc<conf::AppConf> {
    let conf_info = conf::AppConf::load("config/admin/app", config::FileFormat::Yaml).unwrap();

    init_logger(conf_info.clone());

    conf_info
}

pub fn init_logger(_conf: Arc<conf::AppConf>) {
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
