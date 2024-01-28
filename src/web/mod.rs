use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::app::App;

pub async fn run_server() {
	let app = App::get();
	let addr = std::env::var("SPOCK_LISTEN").unwrap_or("0.0.0.0:3000".to_owned());

	let router = Router::new()
		.with_state(app.to_owned())
		.route("/", get(|| async { "Hello, World!" }));

	info!(addr, "Start tcp listener");
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}
