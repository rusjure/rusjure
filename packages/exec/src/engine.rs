use inkwell::context::Context;
use rusjure_ast::Term;
use crate::var::RsjValue;

pub struct ExecEngine {
    context: Context,
}

impl ExecEngine {
    pub fn new() -> Self {
        let context = Context::create();

        Self {
            context,
        }
    }

    pub unsafe fn execute(&mut self, term: &Term) -> RsjValue {
        match term {
            Term::Expr(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::String(_) => todo!(),
            Term::Number(n) => n.into(),
            Term::Float(_) => todo!(),
            Term::Sequence(_) => todo!(),
        }
    }
}
