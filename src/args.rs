use std::{cell::LazyCell, path::PathBuf, sync::Mutex};

use clap::Parser;

pub static ARGS: Mutex<LazyCell<Args>> = Mutex::new(LazyCell::new(|| Args::parse()));

#[derive(Parser, Debug)]
#[clap(author, version, about = "Manage BGP peers with etcd")]
pub struct Args {
    /// Config file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}
