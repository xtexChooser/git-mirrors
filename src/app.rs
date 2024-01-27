use std::sync::Arc;

use parking_lot::RwLock;
use tokio::sync::Notify;

pub struct App {
	pub linter_notify: Notify,
}

impl App {
	pub fn new() -> Arc<Self> {
		Arc::new(Self {
			linter_notify: Notify::const_new(),
		})
	}
}
