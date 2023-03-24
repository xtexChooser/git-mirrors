use std::borrow::BorrowMut;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(transparent)]
pub struct Chain(Vec<Vec<String>>);

pub fn parse_chain(text: &str) -> Result<Chain> {
    let mut lines = vec![];
    for line in text.lines() {
        let mut line = line;
        let parts = line.split('.').map(String::from).collect();
        lines.push(parts);
    }
    Ok(Chain(lines))
}
