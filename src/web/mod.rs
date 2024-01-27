use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::app::App;

pub async fn run_server(app: Arc<App>) {
	let addr = std::env::var("SPOCK_LISTEN").unwrap_or("0.0.0.0:3000".to_owned());

	let router = Router::new().route("/", get(|| async { "Hello, World!" }));

	info!(addr, "http.listen");
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}
