use crate::prelude::*;
use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
use crate::epsilon_duals::perturbations::*;
use crate::differentiation::derivative_ids::*;

use std::iter::zip;
use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
use num::Zero;

#[derive(Debug,Clone)]
pub struct HessianResult<T:Scalar,D:ndarray::Dimension>{
    pub value   :    Array<T,D>,
    pub jacobian:    EArray<T,ndarray::IxDyn>,
    pub hessian :    EArray<T,ndarray::IxDyn>

}

pub fn hessian<'a, T,F,D1,D2>(f: F, x:&EArray<T,D1>) -> HessianResult<T,D2>
where T  : Scalar+'a,
      D1 : ndarray::Dimension,
      D2 : ndarray::Dimension,
      F  : Fn(&EArray<T,D1>) -> EArray<T,D2>
{
    let mut x = x.clone();
    let first_derivative = DerivativeInvocation;
    let first_derivative_id     : u64   = (&first_derivative as *const DerivativeInvocation) as u64;

    for (direction, xi) in x.0.iter_mut().enumerate(){
        let perturbation = Perturbation::<T>::singleton_product(first_derivative_id, direction as u64);
        (*xi).duals = perturbation;
    }

    let mut y = f(&x);

    let second_derivative = DerivativeInvocation;
    let second_derivative_id : u64   = (&first_derivative as *const DerivativeInvocation) as u64;
    for (direction, yi) in y.0.iter_mut().enumerate(){
        let perturbation = Perturbation::<T>::singleton_product(second_derivative_id, direction as u64);
        (*yi).duals = perturbation;
    }

    // let ddf = 


    todo!()
}