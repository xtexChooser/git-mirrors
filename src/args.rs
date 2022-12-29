use std::{
    cell::LazyCell,
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, Result};
use clap::Parser;

pub static ARGS: Mutex<LazyCell<Args>> = Mutex::new(LazyCell::new(Args::parse));

#[derive(Parser, Debug)]
#[clap(author, version, about = "Manage BGP peers with etcd")]
pub struct Args {
    /// Config file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}

pub fn get_args() -> Result<MutexGuard<'static, LazyCell<Args>>> {
    ARGS
        .lock()
        .map_err(|e| anyhow!("failed to lock config {}", e))
}
