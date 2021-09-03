#![no_std]
#![feature(once_cell)]

extern crate alloc;

mod context;
mod data;

pub use context::{BlockchainContext, Context};
pub use data::{get_key, key_to_str, set_key, Dict};
