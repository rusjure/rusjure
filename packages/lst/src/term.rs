use crate::Constant;
use crate::Expression;

pub enum Term<'a> {
    Const(Constant<'a>),
}
