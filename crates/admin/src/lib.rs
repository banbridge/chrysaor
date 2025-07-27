pub mod data;
pub mod domain;
pub mod server;
pub mod service;
pub mod usecase;
pub mod util;

pub mod app;
pub mod conf;
pub mod middleware;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("hello world")
    }
}
