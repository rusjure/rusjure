#![forbid(unsafe_code)]

mod form;
mod token;
mod tokentree;

pub use form::Form;
pub use token::Token;
pub use tokentree::TokenTree;
pub type TokenStream = Vec<TokenTree>;
