#![no_std]

#[path = "lib.rs"]
pub mod lib;

include!(concat!(env!("OUT_DIR"), "/stdlib_bc.rs"));
