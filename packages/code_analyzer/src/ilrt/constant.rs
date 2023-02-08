pub enum Constant<'a> {
    String(&'a str),
    Int(i64),
    Float(f64),
}
