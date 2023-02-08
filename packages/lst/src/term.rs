use crate::Constant;

pub enum Term<'a> {
    Const(Constant<'a>),
}
