use std::ops::*;
use num::{Num, Float};
use num_traits::{One, Zero,Signed};
use core::fmt::Debug;


pub trait Scalar : Debug + Clone + Copy + 'static + 
                   Num + Float + PartialOrd + Neg<Output = Self> +
                   AddAssign + SubAssign + MulAssign + DivAssign +
                   std::iter::Sum + std::iter::Product +
                  {}

// pub trait Scalar: Num {}

use duplicate::duplicate_item;

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Scalar for num_type {}


