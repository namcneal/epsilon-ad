use crate::traits;
use crate::perturbation::Perturbation;

use super::epsilon::EpsilonProduct;
use super::perturbation;


#[derive(Debug, Clone)]
pub enum DualNumber<T: traits::Scalar>{
    Simple(T, Perturbation<T>),
    Complex(Box<DualNumber<T>>, Perturbation<T>)
}

impl<T: traits::Scalar> DualNumber<T>{
    pub fn from_perturbation(perturbation : Perturbation<T>) -> DualNumber<T>{
        Self::Simple(T::zero(), perturbation)

    }
}