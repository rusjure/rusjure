#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct RusjureParser;

#[cfg(test)]
mod tests {
    use pest::Parser;
    use super::*;

    #[test]
    fn test_println_numbers() {
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
