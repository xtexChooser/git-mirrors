use simple_logger::SimpleLogger;

#[tokio::main]
pub async fn main() {
    SimpleLogger::new().init().unwrap();
    info!("peer42d version {}", env!("CARGO_PKG_VERSION"))
}
