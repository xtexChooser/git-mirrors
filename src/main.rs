use simple_logger::SimpleLogger;

#[macro_use]
extern crate log;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("hello")
}
