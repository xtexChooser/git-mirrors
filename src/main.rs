#![feature(imported_main)]
#![feature(once_cell)]
#![feature(is_some_and)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(error_generic_member_access)]

#[macro_use]
extern crate log;

pub mod args;
pub mod config;
pub mod entry;
pub mod etcd;
pub mod peer;
pub mod tunnel;
pub mod util;
pub mod watcher;
pub mod zone;

pub use entry::main;
