use num::Zero;

use crate::epsilon_duals::perturbations::*;
use crate::scalar::Scalar;
use std::iter::repeat;
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

impl<T: Scalar> Dual<T>{
    pub fn zero() -> Self{
        Self::from(T::zero())
    }

    pub fn one() -> Self{
        Self::from(T::one())
    }


    pub fn sqrt(&self) -> Self{
        let mut derived_perturbations = self.duals.clone();
        for c in derived_perturbations.coefficients.iter_mut(){
            *c = <T as From<f64>>::from(0.5) / c.sqrt();
        }

        Dual::<T>{value: self.value.sqrt(), 
                  duals: derived_perturbations
                  }
    }

    pub fn pow(&self, k:usize) -> Dual<T>{
        repeat(self)
            .take(k)
            .fold(Dual::<T>::one(), |acc,item| &acc*item)
    }
}

