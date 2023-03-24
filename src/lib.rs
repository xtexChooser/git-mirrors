#![feature(new_uninit)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(pointer_byte_offsets)]

use serde::{Deserialize, Serialize};

pub mod config;
pub mod inet;
pub mod resolver;
pub mod tun;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(transparent)]
pub struct Chain(Vec<Vec<String>>);
