#[derive(Clone, Debug, thiserror::Error)]
#[error("URL did not match expected domain")]
pub struct DomainError;
