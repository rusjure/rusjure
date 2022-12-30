use rusjure_ast::Term;
use crate::expr::parse_expr;
use crate::parser::Pair;
use crate::parser::Rule;

pub fn parse_term(pair: Pair) -> Term {
    assert_eq!(pair.as_rule(), Rule::Term);
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::Symbol => parse_symbol(inner),
        Rule::Number => parse_number(inner),
        Rule::Expr => Term::Expr(parse_expr(inner)),
        term => unreachable!("All possible inners of term should be listed. Encountered: {:?}", term)
    }
}

fn parse_symbol(pair: Pair) -> Term {
    assert_eq!(pair.as_rule(), Rule::Symbol);
    Term::Symbol(pair.as_str().to_string())
}

fn parse_number(pair: Pair) -> Term {
    assert_eq!(pair.as_rule(), Rule::Number);
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::DecNum => Term::Number(inner.as_str().parse().expect("Number should be already valid")),
        Rule::HexNum => Term::Number(i64::from_str_radix(inner.as_str()
                                                             .trim_start_matches("0x"), 16)
            .expect("Number should be already valid")),
        Rule::BinNum => Term::Number(i64::from_str_radix(inner.as_str()
                                                             .trim_start_matches("0b"), 2)
            .expect("Number should be already valid")),
        Rule::FloatNum => Term::Float(inner.as_str().parse().expect("Number should be already valid")),
        num => unreachable!("All possible inners of term should be listed. Encountered: {:?}", num)
    }
}
