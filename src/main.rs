#![feature(imported_main)]

#[macro_use]
extern crate log;

pub mod entry;
pub use entry::main;
