extern crate rusjure_ast as ast;
extern crate rusjure_lst as lst;

mod ilrt;
mod ilrt_builder;

pub struct LstBuilder {
}

impl LstBuilder {
    pub fn new() -> Self {
        Self {
        }
    }

    /// TODO:
    pub fn b_call<'a>(&mut self, code: &'a ast::Expression) -> lst::Expression<'a> {
        let ast::Expression { first, params } = code;
        let target = match first.as_ref() {
            ast::Term::Symbol(x) => x,
            _ => todo!(),
        };

        let params = params.iter().map(|x| self.b_val(x)).collect::<Vec<_>>();

        lst::Expression::Call {
            target,
            params,
        }
    }

    pub fn b_val<'a>(&mut self, code: &'a ast::Term) -> lst::Term<'a> {
        match code {
            ast::Term::String(x) => lst::Term::Const(lst::Constant::String(x)),
            _ => todo!(),
        }
    }
}

pub fn build_lst(code: &ast::Expression) -> lst::Expression {
    let mut builder = LstBuilder::new();
    builder.b_call(code)
}

#[cfg(test)]
mod tests {
    use crate::LstBuilder;

    #[test]
    fn test_println_hello_world() {
        {
            let ast = ast::Expression {
                first: Box::new(ast::Term::Symbol("foo".to_string())),
                params: vec![],
            };

            let mut builder = LstBuilder::new();
            let lst = builder.b_call(&ast);

            let lst::Expression::Call { target, params, } = lst else { todo!() };
            assert_eq!(target, "foo");
            assert_eq!(params.iter().count(), 0);
        }

        {
            let ast = ast::Expression {
                first: Box::new(ast::Term::Symbol("println".to_string())),
                params: vec![ast::Term::String("Hello world!".to_string())],
            };

            let mut builder = LstBuilder::new();
            let lst = builder.b_call(&ast);

            let lst::Expression::Call { target, params } = lst else { todo!() };
            assert_eq!(target, "println");
            {
                let mut it = params.iter();
                let Some(lst::Term::Const(c)) = it.next() else { todo!() };
                let lst::Constant::String(c) = c else { todo!() };
                assert_eq!(*c, "Hello world!");
                assert!(it.next().is_none());
            }
        }
    }
}
