pub mod app;
pub mod conf;
pub mod data;
pub mod ddd;
pub mod domain;
pub mod middleware;
pub mod server;
pub mod service;
pub mod usecase;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("hello world")
    }
}
