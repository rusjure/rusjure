pub enum RsjValue {
    Int(i64),
}

pub trait IntoRsjValue {
    fn into(self) -> RsjValue;
}

impl<T> From<T> for RsjValue where
        T: IntoRsjValue {
    fn from(x: T) -> Self {
        IntoRsjValue::into(x)
    }
}

impl IntoRsjValue for i64 {
    fn into(self) -> RsjValue {
        RsjValue::Int(self)
    }
}

impl<T> IntoRsjValue for &T where
        T: 'static + Copy + IntoRsjValue {
    fn into(self) -> RsjValue {
        IntoRsjValue::into(*self)
    }
}
