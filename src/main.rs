#![feature(imported_main)]
#![feature(once_cell)]
#![feature(is_some_and)]
#![feature(async_closure)]
#![feature(let_chains)]

#[macro_use]
extern crate log;

pub mod args;
pub mod config;
pub mod entry;
pub mod etcd;
pub mod peer;
pub mod tunnel;
pub mod watcher;
pub mod zone;

pub use entry::main;
