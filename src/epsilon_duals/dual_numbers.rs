use crate::traits;
use crate::perturbation::Perturbation;

#[derive(Debug, Clone)]
pub enum DualNumber<T: traits::Scalar>{
    Unperturbed(T),
    Perturbed(T, Perturbation<T>)
}

impl<T: traits::Scalar> DualNumber<T>{
    pub fn from_perturbation(perturbation : Perturbation<T>) -> DualNumber<T>{
        Self::Perturbed(T::zero(), perturbation)

    }

    pub fn zero() -> DualNumber<T>{
        Self::Unperturbed(T::zero())
    }
}
