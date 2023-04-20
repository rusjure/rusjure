use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub quoted: bool,
    pub tokens: TokenStream,
}
