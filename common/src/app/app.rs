use crate::app::server::Server;

struct Options {
    pub server: Box<dyn Server>,
}

pub struct App {
    options: Options,
}
