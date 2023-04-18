use crate::prelude::*;
use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
use crate::epsilon_duals::perturbations::*;

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
    let ddf  = |x0:&EArray<T,D1>| { 
        jacobian(|a| jacobian(|b| f(b), a).jacobian, x0)
    };  
                

    let result = ddf(&x);
    println!("{:?}", &result);

    todo!()
}