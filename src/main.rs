#[allow(unused_imports)]
use log::{debug, error, warn, info};
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    println!("Hello, world!");
}