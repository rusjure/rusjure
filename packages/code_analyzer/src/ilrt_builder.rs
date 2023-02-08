use ast::Term;
use crate::ilrt;

pub struct IlrtBuilder {
}

impl IlrtBuilder {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn build<'a>(&mut self, code: &'a ast::Expression) -> ilrt::Expression<'a> {
        self.b_expr(code)
    }

    fn b_expr<'a>(&mut self, code: &'a ast::Expression) -> ilrt::Expression<'a> {
        let target = self.b_call_target(&code.first);
        ilrt::Expression::Call {
            target: target.to_string(),
            params: self.b_call_params(&code.params)
        }
    }

    fn b_call_target<'a>(&mut self, term: &'a ast::Term) -> &'a str {
        match term {
            ast::Term::Expr(_) => todo!(),
            ast::Term::Symbol(symbol) => symbol,
            ast::Term::String(_) => todo!(),
            ast::Term::Number(_) => todo!(),
            ast::Term::Float(_) => todo!(),
            ast::Term::Sequence(_) => todo!(),
        }
    }

    fn b_call_params<'a>(&mut self, params: &'a Vec<ast::Term>) -> Vec<ilrt::Term<'a>> {
        let mut res = Vec::with_capacity(params.len());

        for x in params {
            res.push(self.b_call_param(x));
        }

        res
    }

    fn b_call_param<'a>(&mut self, term: &'a ast::Term) -> ilrt::Term<'a> {
        match term {
            ast::Term::Expr(_) => todo!(),
            ast::Term::Symbol(symbol) => todo!(),
            ast::Term::String(string) => ilrt::Term::Constant(ilrt::Constant::String(&string)),
            ast::Term::Number(_) => todo!(),
            ast::Term::Float(_) => todo!(),
            ast::Term::Sequence(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ilrt;
    use crate::ilrt_builder::IlrtBuilder;

    #[test]
    fn test_println_hello_world() {
        let ast = ast::Expression {
            first: Box::new(ast::Term::Symbol("println".to_string())),
            params: vec![
                ast::Term::String("Hello world!".to_string()),
            ],
        };

        let mut builder = IlrtBuilder::new();
        let ilrt = builder.build(&ast);

        let ilrt::Expression::Call {
            target,
            params,
        } = ilrt else { todo!() };

        assert_eq!(target, "println");
        let mut it = params.into_iter();
        let Some(param) = it.next() else { todo!() };
        let ilrt::Term::Constant(param) = param else { todo!() };
        let ilrt::Constant::String(param) = param else { todo!() };
        assert_eq!(param, "Hello world!");
    }
}
