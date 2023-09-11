use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);

#[derive(Parser, Debug)]
#[clap(author, version, about = "Manage BGP peers with etcd")]
pub struct Args {
    /// Config file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}

pub async fn get_args() -> Result<&'static Args> {
    Ok(&*ARGS)
}
