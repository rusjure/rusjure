#![forbid(unsafe_code)]

mod expr;
mod term;

pub use expr::Expression;
pub use term::Term;
pub type Program = Vec<Expression>;
