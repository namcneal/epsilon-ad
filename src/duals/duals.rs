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

    fn sqrt(&self) -> Self{
        let mut derived_perturbations = self.duals.clone();
        for epsilon_product in derived_perturbations.iter_mut(){
            epsilon_product.coefficient = epsilon_product.coefficient;
        }

        Dual::<T>{value: self.value, 
                  duals: derived_perturbations
                  }
    }
}

