use num::Float;

pub trait Scalar : Float {}
impl Scalar for f64 {}