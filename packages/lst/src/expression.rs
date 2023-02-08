use crate::Term;

pub enum Expression<'a> {
    Call {
        target: &'a str,
        params: Vec<Term<'a>>,
    },
}
