use std::{error::Error as StdErr, fmt::Debug, fmt::Display};

use anyhow::{Context, Result};

pub struct SoftError();

impl Debug for SoftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("soft error")
    }
}

impl Display for SoftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("soft error")
    }
}

impl StdErr for SoftError {}

impl SoftError {
    pub fn wrap<T>(result: Result<T>) -> Result<T> {
        result.with_context(|| Self())
    }
}
