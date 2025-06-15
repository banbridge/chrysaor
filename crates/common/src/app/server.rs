pub trait IServer {
    fn start(&self);
    fn stop(&self);
}

pub struct Server {}
