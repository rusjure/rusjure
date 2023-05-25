#![no_std]

#[path = "lib.rs"]
pub mod lib;

pub const BITCODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/corelib.bc"));
