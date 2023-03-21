use crate::epsilon_duals::dual_numbers::*;
use crate::epsilon_duals::perturbation::*;
use crate::traits;


pub trait DerivativeInput<T: traits::Scalar> {
    fn lift_for_differentiation(value:Self, derivative_id:usize, direction:usize) -> DualNumber<T>;
}

impl<T: num_traits::Num + traits::Scalar> DerivativeInput<T> for T{
    fn lift_for_differentiation(value:T, derivative_id:usize, direction:usize) -> DualNumber<T> {
        let perturbation = Perturbation::<T>::singleton(derivative_id, direction);
        DualNumber::<T>::Simple(value, perturbation)
    }
}

impl<T:traits::Scalar> DerivativeInput<T> for DualNumber<T>{
    fn lift_for_differentiation(value:Self, derivative_id:usize, direction:usize) -> DualNumber<T> {
        let perturbation = Perturbation::<T>::singleton(derivative_id, direction);

        DualNumber::<T>::Complex(Box::new(value), perturbation)
    }
}