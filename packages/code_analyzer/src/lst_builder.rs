use crate::ilrt;
use crate::ilrt::{Constant, Expression, Term};

pub struct LstBuilder {
}

impl LstBuilder {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn build<'a>(&mut self, code: &'a ilrt::Expression<'a>) -> lst::Expression<'a> {
        self.b_expr(code)
    }

    fn b_expr<'a>(&mut self, expr: &'a ilrt::Expression<'a>) -> lst::Expression<'a> {
        match expr {
            Expression::Call { target, params } => lst::Expression::Call {
                target,
                params: self.b_call_params(params),
            },
            Expression::BinaryOperation { .. } => todo!(),
        }
    }

    fn b_call_params<'a>(&mut self, params: &'a Vec<ilrt::Term<'a>>) -> Vec<lst::Term<'a>> {
        params.iter()
            .map(|x| self.b_call_param(x))
            .collect()
    }

    fn b_call_param<'a>(&mut self, param: &'a ilrt::Term<'a>) -> lst::Term<'a> {
        match param {
            Term::Constant(c) => lst::Term::Const(self.b_const(c)),
        }
    }

    fn b_const<'a>(&mut self, c: &'a ilrt::Constant<'a>) -> lst::Constant<'a> {
        match c {
            Constant::String(string) => lst::Constant::String(string),
            Constant::Int(_) => todo!(),
            Constant::Float(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ilrt;
    use crate::lst_builder::LstBuilder;

    #[test]
    fn test_println_hello_world() {
        let ilrt = ilrt::Expression::Call {
            target: "println".to_string(),
            params: vec![ilrt::Term::Constant(ilrt::Constant::String("Hello world!"))],
        };

        let mut builder = LstBuilder::new();
        let lst = builder.build(&ilrt);

        let lst::Expression::Call {
            target,
            params,
        } = lst else { todo!() };
        assert_eq!(target, "println");
        let mut it = params.iter();
        let Some(param) = it.next() else { todo!() };
        let lst::Term::Const(param) = param else { todo!() };
        let lst::Constant::String(param) = param else { todo!() };
        assert_eq!(*param, "Hello world!");
    }
}
