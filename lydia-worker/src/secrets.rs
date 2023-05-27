use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash)]
pub struct Secrets {}

unsafe impl Sync for Secrets {}
unsafe impl Send for Secrets {}
