macro_rules! float_eq {
    ($lhs:expr, $rhs:expr) => {
        float_eq!($lhs, $rhs, std::f64::EPSILON)
    };
    ($lhs:expr, $rhs:expr, $epsilon:expr) => {
        ($lhs - $rhs).abs() < $epsilon
    };
}

macro_rules! float_eq_cero {
    ($lhs:expr) => {
        float_eq_cero!($lhs, std::f64::EPSILON)
    };
    ($lhs:expr, $epsilon:expr) => {
        $lhs.abs() < $epsilon
    };
}
