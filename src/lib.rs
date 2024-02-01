#![feature(let_chains)]
#![feature(lazy_cell)]
#![feature(never_type)]

pub mod app;
pub mod db;
pub mod issue;
pub mod linter;
pub mod page;
pub mod rcsyncer;
pub mod web;

#[cfg_attr(feature = "site-mcw", path = "mcw/mod.rs")]
#[cfg_attr(feature = "site-wp", path = "wp/mod.rs")]
pub mod site;
