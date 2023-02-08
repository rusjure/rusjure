#![no_std]

pub use stdlib::*;

mod stdlib;

include!(concat!(env!("OUT_DIR"), "/stdlib_bc.rs"));
