use crate::ilrt::Constant;

pub enum Term<'a> {
    Constant(Constant<'a>),
}
