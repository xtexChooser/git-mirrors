use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(transparent)]
pub struct Chain(pub Vec<Vec<String>>);

impl Chain {
    pub fn len(&self) -> u8 {
        self.0.len() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub fn parse_chain(text: &str) -> Result<Chain> {
    let mut lines = vec![];
    for mut line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut asis_flag = false;
        let mut dns_flag = false;
        while line.starts_with('\\') {
            if line.starts_with("\\asis ") {
                asis_flag = true;
                line = &line[6..];
                break;
            } else if line.starts_with("\\dns ") {
                dns_flag = true;
                if !line.ends_with('.') {
                    bail!("DNS lines in chainfiles must end with a '.' suffix")
                }
                line = &line[5..line.len() - 1];
            }
        }
        if !asis_flag && line.is_empty() {
            continue;
        }
        let parts = if dns_flag {
            line.split('.').map(String::from).collect()
        } else {
            vec![line.to_owned()]
        };
        lines.push(parts);
    }
    lines.reverse();
    if lines.len() > u8::MAX as usize {
        bail!("too many lines in chainfile")
    }
    Ok(Chain(lines))
}
