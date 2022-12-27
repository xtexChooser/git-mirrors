use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

#[derive(Parser, Debug)]
#[clap(author, version, about = "Manage BGP peers with etcd")]
pub struct Args {
    /// Config file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}
