use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(transparent)]
pub struct Chain(Vec<Vec<String>>);

pub fn parse_chain(text: &str) -> Result<Chain> {
    let mut lines = vec![];
    for mut line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut dns_flag = false;
        while line.starts_with('\\') {
            if line.starts_with("\\asis ") {
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
        let parts = if dns_flag {
            line.split('.').map(String::from).collect()
        } else {
            vec![line.to_owned()]
        };
        lines.push(parts);
    }
    Ok(Chain(lines))
}
