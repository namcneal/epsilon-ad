use crate::epsilon_duals::dual_numbers::DualNumber;
use crate::differentiation::lift_for_differentiation::*;
use crate::traits;

use ndarray::{Array1, ArrayD};

pub fn jacobian<T: traits::Scalar>(f: fn(Array1<DualNumber<T>>) -> Array1<DualNumber<T>>, x:Array1<T>) -> (){
    let derivative_called : bool        = true; 
    let derivative_id     : *const bool = &derivative_called;
    let derivative_id     : usize       = derivative_id as usize;

    let lifted_x = <Array1<T> as LiftArray<T, DualNumber<T>>>::lift_for_differentiation(x, derivative_id);

    let result =  f(lifted_x);
    println!("{:?}", result);
}


