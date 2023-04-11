use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    quoted: bool,
    tokens: TokenStream,
}
