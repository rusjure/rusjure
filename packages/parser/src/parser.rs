use crate::expr::parse_expr;
use pest::Parser;
use rusjure_ast::Program;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct RusjureParser;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub fn parse(source: &str) -> Result<Program, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = RusjureParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(parse_expr(pair));
        }
    }
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use rusjure_ast::{Expression, Term};
    use super::*;

    #[test]
    fn test_println_numbers() {
        let ast = parse(r#"(println 123)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![Term::Number(123)]
        });

        let ast = parse(r#"(println -123)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![Term::Number(-123)]
        });

        let ast = parse(r#"(println 0x12FA3)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![Term::Number(0x12FA3)]
        });

        let ast = parse(r#"(+ 3.14 0.5 .5 15.)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("+".to_string())),
            params: vec![Term::Float(3.14), Term::Float(0.5), Term::Float(0.5), Term::Float(15.0)]
        });

        let ast = parse(r#"(- 3.14 -0.5 .5 -15. 15.025)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("-".to_string())),
            params: vec![Term::Float(3.14), Term::Float(-0.5), Term::Float(0.5), Term::Float(-15.0), Term::Float(15.025)]
        });

        let ast = parse(r#"(println 0b1001001)"#).expect("Failed to parse.");
        assert_eq!(ast.into_iter().next().unwrap(),
        Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![Term::Number(0b1001001)]
        });
    }

    #[test]
    fn test_pest_println_numbers() {
        {
            let expr = RusjureParser::parse(Rule::Program, r#"(println 123)"#).expect("Failed to parse")
                .next().unwrap();
            assert_eq!(expr.as_rule(), Rule::Expr);
            let mut inner = expr.into_inner();

            let first = inner.next().unwrap();
            assert_eq!(first.as_rule(), Rule::Term);
            let first = first.into_inner().next().unwrap();
            assert_eq!(first.as_rule(), Rule::Symbol);
            assert_eq!(first.as_str(), "println");

            let second = inner.next().unwrap();
            assert_eq!(second.as_rule(), Rule::Term);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::Number);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::DecNum);
            assert_eq!(second.as_str(), "123");
        }

        {
            let expr = RusjureParser::parse(Rule::Program, r#"(println 0b01001)"#).expect("Failed to parse")
                .next().unwrap();
            assert_eq!(expr.as_rule(), Rule::Expr);
            let mut inner = expr.into_inner();

            let first = inner.next().unwrap();
            assert_eq!(first.as_rule(), Rule::Term);
            let first = first.into_inner().next().unwrap();
            assert_eq!(first.as_rule(), Rule::Symbol);
            assert_eq!(first.as_str(), "println");

            let second = inner.next().unwrap();
            assert_eq!(second.as_rule(), Rule::Term);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::Number);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::BinNum);
            assert_eq!(second.as_str(), "0b01001");
        }

        {
            let expr = RusjureParser::parse(Rule::Program, r#"(println 0x5A56F32D)"#).expect("Failed to parse")
                .next().unwrap();
            assert_eq!(expr.as_rule(), Rule::Expr);
            let mut inner = expr.into_inner();

            let first = inner.next().unwrap();
            assert_eq!(first.as_rule(), Rule::Term);
            let first = first.into_inner().next().unwrap();
            assert_eq!(first.as_rule(), Rule::Symbol);
            assert_eq!(first.as_str(), "println");

            let second = inner.next().unwrap();
            assert_eq!(second.as_rule(), Rule::Term);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::Number);
            let second = second.into_inner().next().unwrap();
            assert_eq!(second.as_rule(), Rule::HexNum);
            assert_eq!(second.as_str(), "0x5A56F32D");
        }
    }
}
