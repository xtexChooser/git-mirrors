#![feature(imported_main)]
#![feature(once_cell)]

#[macro_use]
extern crate log;

pub mod args;
pub mod config;
pub mod entry;

pub use entry::main;
