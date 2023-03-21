use crate::epsilon_duals::dual_numbers::DualNumber;
use crate::differentiation::lift_for_differentiation::*;
use crate::traits;

use ndarray::{Array1, ArrayD};

type JacobianTensor<T>  = ArrayD<T>;

pub fn jacobian<T,F>(f: F, x:Array1<T>) -> JacobianTensor<T>
where T : traits::Scalar,
       F: FnOnce(&dyn LiftableArray<DualNumber<T>>) -> &dyn LiftableArray<DualNumber<T>>
{
    let _jacobian : ArrayD<T>;

    let derivative_called : bool        = true; 
    let derivative_id     : *const bool = &derivative_called;
    let derivative_id     : usize       = derivative_id as usize;

    let lifted_x = LiftableArray::<DualNumber<T>>::lift_for_differentiation(x, derivative_id);

    let result =  f(&lifted_x as &dyn LiftableArray<DualNumber<T>>) ;

    _jacobian
    // println!("{:?}", result);
}


