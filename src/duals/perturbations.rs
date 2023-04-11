
use crate::duals::epsilons::*;
use crate::scalar::Scalar;
use smallvec::SmallVec;
use std::ops::*;

const SMALL_VEC_NUMBER_ELEMENTS : usize = 128;
pub (super) const SVNE : usize = SMALL_VEC_NUMBER_ELEMENTS;
pub (super) type PerturbationData<T> = SmallVec<[T; SVNE]>;


#[derive(Debug, Clone, PartialEq)]
pub struct Perturbation<T: Scalar>{
    pub coefficients : PerturbationData<T> ,
    pub products     : PerturbationData<NonEmptyEpsilonProduct>
}

impl<T: Scalar> Perturbation<T>{
    pub fn empty_perturbation()->Perturbation<T>{
        Perturbation::<T>{ coefficients: PerturbationData::<T>::new(), 
                           products    : PerturbationData::<NonEmptyEpsilonProduct>::new()}
    }

    pub fn singleton_product(invocation_id:u64, direction:u64) -> Perturbation<T>{
        let mut perturbation = Perturbation::<T>::empty_perturbation();
        perturbation.coefficients.push(T::one());
        perturbation.products.push(NonEmptyEpsilonProduct::singleton(invocation_id, direction));

        return perturbation
    }
}
