use crate::ilrt::operator::BinaryOperator;
use crate::ilrt::term::Term;

pub enum Expression<'a> {
    Call {
        target: String,
        params: Vec<Term<'a>>,
    },
    BinaryOperation {
        left: Term<'a>,
        right: Term<'a>,
        operator: BinaryOperator,
    },
}
