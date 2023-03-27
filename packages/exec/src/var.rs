use either::Either;

pub enum RsjValue {
    Int(i64),
    Float(f64),
}

pub trait IntoRsjValue {
    fn into_rsj_value(self) -> RsjValue;
}

impl<T> From<T> for RsjValue where
        T: IntoRsjValue {
    fn from(x: T) -> Self {
        IntoRsjValue::into_rsj_value(x)
    }
}

impl IntoRsjValue for i64 {
    fn into_rsj_value(self) -> RsjValue {
        RsjValue::Int(self)
    }
}

impl IntoRsjValue for f64 {
    fn into_rsj_value(self) -> RsjValue {
        RsjValue::Float(self)
    }
}

impl<T1, T2> IntoRsjValue for Either<T1, T2> where
        T1: IntoRsjValue,
        T2: IntoRsjValue {
    fn into_rsj_value(self) -> RsjValue {
        match self {
            Either::Left(x) => x.into_rsj_value(),
            Either::Right(x) => x.into_rsj_value(),
        }
    }
}

impl<T> IntoRsjValue for &T where
        T: 'static + Copy + IntoRsjValue {
    fn into_rsj_value(self) -> RsjValue {
        IntoRsjValue::into_rsj_value(*self)
    }
}
