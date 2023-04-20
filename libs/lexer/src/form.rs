use crate::lexer::Pair;
use crate::lexer::Rule;
use rusjure_tokens::Form;
use rusjure_tokens::TokenTree;

use crate::lexer::FromPest;

impl FromPest for Form {
    fn from_pest(pair: Pair) -> Self {
        assert_eq!(pair.as_rule(), Rule::Form);
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();
        match first.as_rule() {
            Rule::QuotedForm => quoted_from_pest(first),
            Rule::NonQuotedForm => nonquoted_from_pest(first),
            _ => unreachable!(),
        }
    }
}

fn quoted_from_pest(pair: Pair) -> Form {
    assert_eq!(pair.as_rule(), Rule::QuotedForm);
    let mut it = pair.into_inner();
    let first = it.next().unwrap();
    let mut form = nonquoted_from_pest(first);
    form.quoted = true;
    form
}

fn nonquoted_from_pest(pair: Pair) -> Form {
    assert_eq!(pair.as_rule(), Rule::NonQuotedForm);
    let inner = pair.into_inner();
    let inner = inner.map(TokenTree::from_pest).collect::<Vec<_>>();
    Form {
        quoted: false,
        tokens: inner,
    }
}
