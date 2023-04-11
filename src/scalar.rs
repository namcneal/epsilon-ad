use std::ops::*;
use num::{Num, Float};
use num_traits::{One, Zero,};
use core::fmt::Debug;


pub trait Scalar : Debug + Clone + Copy + 'static + 
                   Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
                   AddAssign + MulAssign + Neg<Output=Self> + Zero + One +
                   Num {}

// pub trait Scalar: Num {}

use duplicate::duplicate_item;

#[duplicate_item(
    num_type;
    [f32];
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Scalar for num_type {}


