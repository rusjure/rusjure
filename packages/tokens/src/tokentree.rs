use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Form(Form),
    Sequence(TokenStream),
    Token(Token),
}
