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
        match pair.as_rule() {
            Rule::TokenTree => {
                let tt = rusjure_tokens::TokenTree::from_pest(pair);
                ast.push(tt);
            }
            Rule::EOI => {
                break;
            }
            _ => unreachable!(),
        }
    }
    Ok(ast)
}

pub trait FromPest {
    fn from_pest(pair: Pair) -> Self;
}
