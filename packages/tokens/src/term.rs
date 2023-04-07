use crate::expr::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Expr(Expression),
    Symbol(String),
    String(String),
    Number(i64),
    Float(f64),
    Sequence(Vec<Term>),
}
