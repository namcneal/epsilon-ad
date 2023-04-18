use std::ops::*;
use num::{Num, Float};
use num_traits::{One, Zero,Signed};
use core::fmt::Debug;


pub trait Scalar : Debug + Clone + Copy + 'static + 
                   Float + Signed  + Zero + One + 
                   AddAssign + SubAssign + MulAssign + DivAssign +
                   std::iter::Sum +
                   From<f64>
                  {}

// pub trait Scalar: Num {}

use duplicate::duplicate_item;

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Scalar for num_type {}


