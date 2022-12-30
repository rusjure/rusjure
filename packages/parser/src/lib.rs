#![forbid(unsafe)]

mod expr;
mod parser;
mod term;

pub use parser::parse;
