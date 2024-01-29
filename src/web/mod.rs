use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::app::App;

pub mod meta;

pub async fn run_server() {
	let app = App::get();
	let addr = std::env::var("SPOCK_LISTEN").unwrap_or("0.0.0.0:3000".to_owned());

	let router = Router::new()
		.with_state(app.to_owned())
		.merge(meta::new_router());

	info!(addr, "Start tcp listener");
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}
// https://modrinth.com/auth/authorize?client_id=CnydSoVX&redirect_uri=http%3A%2F%2F127%2E0%2E0%2E1%3A3000%2Fauth&scope=USER_READ+USER_READ_EMAIL