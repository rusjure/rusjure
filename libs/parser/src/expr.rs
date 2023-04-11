use crate::parser::Pair;
use crate::parser::Rule;
use crate::term::parse_term;
use rusjure_tokens::Expression;

/// Maps the `pair` into an AST expression.
/// Panics if not valid.
pub fn parse_expr(pair: Pair) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Expr);
    let mut it = pair.into_inner().map(parse_term);
    let first = it.next().expect("Expression never has no first element.");
    let params = it.collect::<Vec<_>>();
    Expression {
        first: Box::new(first),
        params,
    }
}
