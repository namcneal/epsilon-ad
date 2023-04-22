use num::Zero;

use crate::epsilon_duals::perturbations::*;
use crate::scalar::Scalar;
use std::iter::repeat;
use std::ops::*;

#[derive(Clone, PartialEq)]
pub struct Dual<T: Scalar>{
    pub value : T,
    pub duals : Perturbation<T>
}

impl<T: Scalar> std::fmt::Debug for Dual<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = String::new();
        representation.push_str(
            &format!("{:?}", self.duals)
        );
        representation.push_str(
            &format!("{:?}", self.value)
        );

        write!(f, "{}", &representation)

    }
}

impl<T: Scalar> From<T> for Dual<T>{
    fn from(value: T) -> Self {
        Dual::<T> { value: value, 
                    duals: Perturbation::<T>::empty_perturbation()}
        }
}

impl<T: Scalar> From<Perturbation<T>> for Dual<T>{
    fn from(value: Perturbation<T>) -> Self {
        Dual::<T> { value: T::zero(), 
                    duals: value}
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
            *c = T::from(0.5).unwrap() / c.sqrt();
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

