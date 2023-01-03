use crate::Constant;
use crate::Expression;

pub enum Term<'a> {
    Expr(Box<Expression<'a>>),
    Const(Constant<'a>),
}
