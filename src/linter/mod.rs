use std::sync::Arc;

use tracing::info;

use crate::app::App;

pub async fn run_linter(app: Arc<App>) {
	loop {
		tokio::select! {
			_ = app.linter_notify.notified()=>{},
			_ = tokio::time::sleep(std::time::Duration::from_secs(120))=>{}
		}
		info!("sync");
	}
}
