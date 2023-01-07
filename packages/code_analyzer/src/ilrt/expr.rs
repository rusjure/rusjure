use crate::ilrt::term::Term;

pub struct Expression<'a> {
    target: String,
    params: Vec<Term<'a>>,
}
