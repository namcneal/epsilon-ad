use std::ops::*;
use std::fmt::Debug;
use num_traits::*;

pub trait Scalar: Sized + Debug + Copy + Clone + One + Zero + 
              Add<Output=Self> + Mul<Output=Self> + 
              {}


impl Scalar for f64 {} 