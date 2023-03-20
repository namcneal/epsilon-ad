use std::ops::*;
use num_traits::*;

pub trait Scalar: Sized + Copy + Clone + One + 
              Add<Output=Self> + Mul<Output=Self> + 
              {}