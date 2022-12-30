use crate::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub first: Box<Term>,
    pub params: Vec<Term>,
}
