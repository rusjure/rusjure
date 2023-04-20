#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(String),
    String(String),
    Number(i64),
    Float(f64),
}
