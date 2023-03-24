use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Chain;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ResolverConfig {
    pub path: String,
    pub format_width: u16,
}

pub trait Resolver {
    fn resolve(&self, id: u128) -> Result<Option<Chain>>;
}

impl ResolverConfig {
    pub fn to_full_path(&self, id: u128) -> String {
        format!(
            "{}{:0width$x}",
            self.path,
            id,
            width = self.format_width as usize
        )
    }
}

impl Resolver for ResolverConfig {
    fn resolve(&self, id: u128) -> Result<Option<Chain>> {
        println!("{}", self.to_full_path(0xcfcf));
        todo!()
    }
}
