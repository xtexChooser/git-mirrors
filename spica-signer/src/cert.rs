use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CertConfig {
    pub file: String,
}
