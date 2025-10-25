use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::rfc_3339()) // 使用 RFC3339 格式的本地时间
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_target(false), //    .json()
        )
        .init()
}
