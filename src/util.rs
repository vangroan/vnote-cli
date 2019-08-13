/// Inexact equality comparison
/// for floating point numbers.
#[macro_export]
macro_rules! inexact_eq {
    ($lhs:expr, $rhs:expr) => {
        ($lhs as f64 - $rhs as f64).abs() < ::std::f64::EPSILON
    };
}
