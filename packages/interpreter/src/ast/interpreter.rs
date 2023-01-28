use rusjure_ast::Term;
use crate::ast::val::RsjValue;

pub struct Interpreter {
}

impl Interpreter {
    pub fn eval(&self, term: &Term) -> RsjValue {
        match term {
            Term::Expr(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::String(_) => todo!(),
            Term::Number(n) => RsjValue::Int(*n),
            Term::Float(f) => RsjValue::Float(*f),
            Term::Sequence(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rusjure_ast::Term;
    use crate::ast::interpreter::Interpreter;
    use crate::ast::val::RsjValue;

    #[test]
    fn provide_const_int() {
        let a = 15;
        let b = 30;

        let interpreter = Interpreter {};
        let RsjValue::Int(res_a) = interpreter.eval(&Term::Number(a)) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::Int(res_b) = interpreter.eval(&Term::Number(b)) else { todo!() };
        assert_eq!(res_b, b);
    }

    #[test]
    fn provide_const_float() {
        let a = 15.0;
        let b = 30.5;

        let interpreter = Interpreter {};
        let RsjValue::Float(res_a) = interpreter.eval(&Term::Float(a)) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::Float(res_b) = interpreter.eval(&Term::Float(b)) else { todo!() };
        assert_eq!(res_b, b);
    }
}
