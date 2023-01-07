use crate::ilrt::Constant;
use crate::ilrt::expr::Expression;

pub enum Term<'a> {
    Expression(Expression<'a>),
    Constant(Constant<'a>),
}
