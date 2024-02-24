use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use trust_dns_server::{client::rr::Label, proto::rr::Name};

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

    pub fn name(&self, line: usize) -> Result<Name> {
        if let Some(line) = self.0.get(line) {
            let mut name = Name::new();
            if line.len() == 1 {
                let line = &line[0];
                name = name.append_label(Label::from_raw_bytes(line.as_bytes())?)?;
            } else {
                for part in line.iter() {
                    name = name.append_label(part.as_str())?;
                }
            }
            Ok(name)
        } else {
            bail!("line not found")
        }
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
        if !asis_flag {
            if line.is_empty() {
                continue;
            }
            if let Err(e) = Label::from_utf8(line) {
                bail!(format!("line check failed: {e} {line}"))
            }
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
