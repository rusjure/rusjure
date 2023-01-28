use rusjure_ast::Term;
use crate::ast::val::RsjValue;

pub struct Interpreter {
}

impl Interpreter {
    pub fn eval(&self, term: &Term) -> RsjValue {
        match term {
            Term::Expr(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::String(str) => RsjValue::String(str.to_string()),
            Term::Number(n) => RsjValue::Int(*n),
            Term::Float(f) => RsjValue::Float(*f),
            Term::Sequence(seq) => RsjValue::Sequence(seq.iter().map(|term| self.eval(term)).collect()),
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

    #[test]
    fn provide_const_string() {
        let a = "foobar";
        let b = "foobarbaz";

        let interpreter = Interpreter {};
        let RsjValue::String(res_a) = interpreter.eval(&Term::String(a.to_string())) else { todo!() };
        assert_eq!(res_a, a);

        let RsjValue::String(res_b) = interpreter.eval(&Term::String(b.to_string())) else { todo!() };
        assert_eq!(res_b, b);
    }

    #[test]
    fn provide_const_sequence() {
        let seq = vec![Term::Number(15), Term::String("foo".to_string())];

        let interpreter = Interpreter {};
        let RsjValue::Sequence(iseq) = interpreter.eval(&Term::Sequence(seq)) else { todo!() };
        let mut it = iseq.iter();

        let Some(RsjValue::Int(num)) = it.next() else { todo!() };
        assert_eq!(*num, 15);

        let Some(RsjValue::String(str)) = it.next() else { todo!() };
        assert_eq!(str, "foo");

        assert_eq!(it.next(), None);
    }
}
