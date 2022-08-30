use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Utilities to manage peerings with configuration files."
)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,

    /// Config file
    #[clap(short, long, value_parser)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Run as a daemon process
    Daemon,
    /// Run once, inotify will get disabled
    Apply,
    /// Show status
    Show,
}
