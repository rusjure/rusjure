use std::collections::HashMap;
use std::collections::VecDeque;

use rusjure_ast::{Term, Expression};
use crate::ast::val::RsjValue;

use super::val::RsjFunction;

pub struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn eval(&self, term: &Term) -> RsjValue {
        match term {
            Term::Expr(Expression { first, params }) => {
                let target = self.eval(first);
                let params = match params.len() {
                    0 => RsjValue::Nil,
                    1 => self.eval(params.first().unwrap()),
                    _ => RsjValue::Sequence(params.iter().map(|x| self.eval(x)).collect()),
                };
                self.exec(target, params)
            },
            Term::Symbol(s) => self.provide_symbol(s),
            Term::String(str) => RsjValue::String(str.to_string()),
            Term::Number(n) => RsjValue::Int(*n),
            Term::Float(f) => RsjValue::Float(*f),
            Term::Sequence(seq) => RsjValue::Sequence(seq.iter().map(|term| self.eval(term)).collect()),
        }
    }

    fn exec(&self, target: RsjValue, params: RsjValue) -> RsjValue {
        match target {
            RsjValue::Function(function) => match function {
                RsjFunction::External(f) => f(params),
            },
            _ => todo!(),
        }
    }

    fn provide_symbol(&self, symbol: &str) -> RsjValue {
        match symbol {
            "println" => RsjValue::Function(RsjFunction::External(Box::new(println))),
            _ => todo!(),
        }
    }
}

fn println(params: RsjValue) -> RsjValue {
    match params {
        RsjValue::String(s) => println!("{}", s),
        _ => todo!(),
    };
    RsjValue::Nil
}

#[cfg(test)]
mod tests {
    use rusjure_ast::{Term, Expression};
    use crate::ast::interpreter::Interpreter;
    use crate::ast::val::RsjValue;

    #[test]
    fn provide_const_int() {
        let a = 15;
        let b = 30;

        let interpreter = Interpreter::new();
        let RsjValue::Int(res_a) = interpreter.eval(&Term::Number(a)) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::Int(res_b) = interpreter.eval(&Term::Number(b)) else { todo!() };
        assert_eq!(res_b, b);
    }

    #[test]
    fn provide_const_float() {
        let a = 15.0;
        let b = 30.5;

        let interpreter = Interpreter::new();
        let RsjValue::Float(res_a) = interpreter.eval(&Term::Float(a)) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::Float(res_b) = interpreter.eval(&Term::Float(b)) else { todo!() };
        assert_eq!(res_b, b);
    }

    #[test]
    fn provide_const_string() {
        let a = "foobar";
        let b = "foobarbaz";

        let interpreter = Interpreter::new();
        let RsjValue::String(res_a) = interpreter.eval(&Term::String(a.to_string())) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::String(res_b) = interpreter.eval(&Term::String(b.to_string())) else { todo!() };
        assert_eq!(res_b, b);
    }

    #[test]
    fn provide_const_sequence() {
        let seq = vec![Term::Number(15), Term::String("foo".to_string())];

        let interpreter = Interpreter::new();
        let RsjValue::Sequence(iseq) = interpreter.eval(&Term::Sequence(seq)) else { todo!() };
        let mut it = iseq.iter();

        let Some(RsjValue::Int(num)) = it.next() else { todo!() };
        assert_eq!(*num, 15);

        let Some(RsjValue::String(str)) = it.next() else { todo!() };
        assert_eq!(str, "foo");

        assert_eq!(it.next(), None);
    }

    #[test]
    fn print_hello_world() {
        let expr = Term::Expr(Expression {
            first: Box::new(Term::Symbol("println".to_string())),
            params: vec![Term::String("Hello world!".to_string())]
        });

        let interpreter = Interpreter::new();
        assert_eq!(interpreter.eval(&expr), RsjValue::Nil);
    }
}
