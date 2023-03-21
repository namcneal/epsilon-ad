use crate::epsilon_duals::dual_numbers::*;
use crate::epsilon_duals::perturbation::*;
use crate::traits;

use ndarray::{Array, Array1};

pub trait LiftableArray<T: traits::Scalar>{
    fn lift_for_differentiation(self:Self, derivative_id:usize) -> Array1<DualNumber<T>>;
}


impl<T: traits::Scalar> LiftableArray<T> for Array1<T>{
    
    fn lift_for_differentiation(self:Self, derivative_id:usize) -> Array1<DualNumber<T>> {
        let mut lifted : Array1<DualNumber<T>> = Array::from_elem((self.len(),), DualNumber::<T>::zero());

        for (direction, xi) in self.iter().enumerate(){
            let perturbation = Perturbation::<T>::singleton(derivative_id, direction);
            lifted[direction] = DualNumber::<T>::Perturbed(*xi, perturbation);
        }

        return lifted
    }
}

impl<T:traits::Scalar> LiftableArray<T> for Array1<DualNumber<T>>{
    
    fn lift_for_differentiation(self:Self, derivative_id:usize) -> Array1<DualNumber<T>> {
        let mut lifted : Array1<DualNumber<T>> = Array::from_elem((self.len(),), DualNumber::<T>::zero());

        for (direction, xi) in self.iter().enumerate(){

            let new_perturbation = Perturbation::<T>::singleton(derivative_id, direction);

            match xi{
                DualNumber::Unperturbed(value) => {
                    lifted[direction] = DualNumber::Perturbed(*value, new_perturbation)
                },

                DualNumber::Perturbed(value, perturbation) => {
                    lifted[direction] = DualNumber::Perturbed(*value, new_perturbation + (*perturbation).clone())
                }

            }
        }

        return lifted
    }
}
