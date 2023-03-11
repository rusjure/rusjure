use either::{Left, Right};
use inkwell::context::Context;
use rusjure_ast::{Expression, Term};
use crate::IntoRsjValue;
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
            Term::Expr(expression) => self.invoke(expression),
            Term::Symbol(_) => todo!(),
            Term::String(_) => todo!(),
            Term::Number(n) => n.into(),
            Term::Float(x) => x.into(),
            Term::Sequence(_) => todo!(),
        }
    }

    pub unsafe fn invoke(&mut self, expression: &Expression) -> RsjValue {
        let Expression { first, params } = expression;
        let target = match &**first {
            Term::Symbol(symbol) => symbol,
            Term::Expr(_expr) => todo!(),
            Term::Sequence(_seq) => todo!(),
            _ => todo!(),
        };
        match target.as_str() {
            "+" => {
                IntoRsjValue::into_rsj_value(params.iter().fold(Left(0),
                                                                |acc, curr| match curr {
                    Term::Expr(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::String(_) => todo!(),
                    Term::Number(n) => acc
                        .map_left(|x| x + n)
                        .map_right(|x: f64| x + (*n as f64)),
                    Term::Float(y) => match acc {
                        Left(x) => Right(x as f64 + y),
                        Right(x) => Right(x + y),
                    },
                    Term::Sequence(_) => todo!(),
                }))
            },
            _ => todo!(),
        }
    }
}
