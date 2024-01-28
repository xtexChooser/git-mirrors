use crate::app::App;

pub async fn run_linter() {
	let app = App::get();

	let _ = app.mwbot("zh").await.unwrap();
	let _ = app.mwbot("en").await.unwrap();

	loop {
		tokio::select! {
			_ = app.linter_notify.notified()=>{},
			_ = tokio::time::sleep(std::time::Duration::from_secs(120))=>{}
		}
	}
}
