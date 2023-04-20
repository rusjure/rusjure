use crate::lexer::Pair;
use crate::lexer::Rule;
use rusjure_tokens::Token;

use crate::lexer::FromPest;

impl FromPest for Token {
    fn from_pest(pair: Pair) -> Self {
        assert_eq!(pair.as_rule(), Rule::Token);
        let mut inner = pair.into_inner();
        let inner = inner.next().unwrap();
        match inner.as_rule() {
            // String | Float | Int
            Rule::Float => todo!(),
            Rule::Int => todo!(),
            Rule::String => string_from_pest(inner),
            Rule::Symbol => symbol_from_pest(inner),
            _ => unreachable!(),
        }
    }
}

fn symbol_from_pest(pair: Pair) -> Token {
    assert_eq!(pair.as_rule(), Rule::Symbol);
    Token::Symbol(pair.as_str().to_string())
}

fn string_from_pest(pair: Pair) -> Token {
    assert_eq!(pair.as_rule(), Rule::String);
    let mut inner = pair.into_inner();
    let inner = inner.next().unwrap();
    Token::String(inner.as_str().to_string())
}
