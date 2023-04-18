use pest::Parser;
use rusjure_tokens::TokenStream;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct RusjureLexer;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub fn parse(source: &str) -> Result<TokenStream, Box<pest::error::Error<Rule>>> {
    let mut ast = vec![];
    let pairs = RusjureLexer::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            // ast.push(parse_expr(pair));
            todo!();
        }
    }
    Ok(ast)
}
