use std::marker::PhantomData;

use crate::prelude::*;
use crate::Scalar;
use crate::differentiation::differentiate::*;

// use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};

// use std::iter::zip;
// use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
// use num::Zero;

#[derive(Debug,Clone)]
pub struct JacobianResult<T:Scalar,D:ndarray::Dimension>{
    pub value   :    ndarray::Array<T,D>,
    pub jacobian:    ndarray::Array<T,ndarray::IxDyn>,
}


pub fn jacobian<T,F,D>(f: F, x:&ndarray::Array1<T>) -> JacobianResult<T, D>
where T : Scalar,
      D : ndarray::Dimension,
      F : Fn(&EVector<T>) -> EArray<T,D>
{   
    const order : usize = 1;
    let derivative_scheduled = DerivativeInvocation::<T,order>::new(x.clone());
    let called_with_epsilon_result = derivative_scheduled.call(f);

    let mut value_and_jacobian = called_with_epsilon_result.extract_all_derivatives();
    assert!(value_and_jacobian.1.len() == 1);

    JacobianResult::<T,D>{value: value_and_jacobian.0, jacobian: value_and_jacobian.1.remove(0)}
}
