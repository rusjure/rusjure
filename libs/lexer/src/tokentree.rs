use crate::lexer::Rule;
use rusjure_tokens::Form;
use rusjure_tokens::Token;
use rusjure_tokens::TokenTree;

use crate::lexer::FromPest;

impl FromPest for TokenTree {
    fn from_pest(pair: crate::lexer::Pair) -> Self {
        assert_eq!(pair.as_rule(), Rule::TokenTree);
        let inner = {
            let mut it = pair.into_inner();
            let inner = it.next().unwrap();
            assert_eq!(it.next(), None);
            inner
        };
        match inner.as_rule() {
            Rule::Form => TokenTree::Form(Form::from_pest(inner)),
            Rule::Sequence => todo!(),
            Rule::Token => TokenTree::Token(Token::from_pest(inner)),
            _ => unreachable!(),
        }
    }
}
