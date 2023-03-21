use crate::epsilon_duals::dual_numbers::*;
use crate::epsilon_duals::perturbation::*;
use crate::traits;

use ndarray::{Array, Array1};

pub trait LiftArray<InT, OutT> {
    fn lift_for_differentiation(to_lift:Array1<InT>, derivative_id:usize) -> Array1<OutT>;
}


impl<T: traits::Scalar> LiftArray<T, DualNumber<T>> for Array1<T>{
    
    fn lift_for_differentiation(to_lift:Array1<T>, derivative_id:usize) -> Array1<DualNumber<T>> {
        let mut lifted : Array1<DualNumber<T>> = Array::from_elem((to_lift.len(),), DualNumber::<T>::zero());

        for (direction, xi) in to_lift.iter().enumerate(){
            let perturbation = Perturbation::<T>::singleton(derivative_id, direction);
            lifted[direction] = DualNumber::<T>::Perturbed(*xi, perturbation);
        }

        return lifted
    }
}

impl<T:traits::Scalar> LiftArray<DualNumber<T>, DualNumber<T>> for DualNumber<T>{
    
    fn lift_for_differentiation(to_lift:Array1<DualNumber<T>>, derivative_id:usize) -> Array1<DualNumber<T>> {
        let mut lifted : Array1<DualNumber<T>> = Array::from_elem((to_lift.len(),), DualNumber::<T>::zero());

        for (direction, xi) in to_lift.iter().enumerate(){

            let new_perturbation = Perturbation::<T>::singleton(derivative_id, direction);

            match xi{
                Self::Unperturbed(value) => {
                    lifted[direction] = Self::Perturbed(*value, new_perturbation)
                },

                Self::Perturbed(value, perturbation) => {
                    lifted[direction] = Self::Perturbed(*value, new_perturbation + (*perturbation).clone())
                }

            }
        }

        return lifted
    }
}
