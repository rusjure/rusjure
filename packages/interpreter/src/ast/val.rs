use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RsjValue {
    Nil,
    Int(i64),
    Float(f64),
    String(String),
    Sequence(Vec<RsjValue>),
    Function(RsjFunction),
}

pub enum RsjFunction {
    External(Box<dyn Fn(RsjValue) -> RsjValue>),
}

impl Debug for RsjFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for RsjFunction {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
