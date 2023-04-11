use num::Zero;

use crate::duals::perturbations::*;
use crate::scalar::Scalar;
use std::ops::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Dual<T: Scalar>{
    pub value : T,
    pub duals : Perturbation<T>
}

impl<T: Scalar> From<T> for Dual<T>{
    fn from(value: T) -> Self {
        Dual::<T> { value: value, 
                    duals: Perturbation::<T>::empty_perturbation()}
    }
}

