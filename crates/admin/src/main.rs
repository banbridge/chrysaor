#[allow(unused_imports)]
use admin::app;

#[tokio::main]
async fn main() {
    let mut ctx = rudi::Context::auto_register();
    ctx.resolve_async::<()>().await
}
