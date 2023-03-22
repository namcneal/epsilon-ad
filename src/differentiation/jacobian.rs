use crate::epsilon_duals::dual_numbers::DualNumber;
use crate::differentiation::lifting::*;
use crate::traits;

use ndarray::{Array1, ArrayD};


pub fn jacobian<T,F>(f: F, x:Array1<T>) -> ()
where T : traits::Scalar + 'static,
      F : Fn(&Array1<DualNumber<T>>) -> Array1<DualNumber<T>>
{
    let _jacobian : ArrayD<T>;

    let derivative_called : bool        = true; 
    let derivative_id     : *const bool = &derivative_called;
    let derivative_id     : usize       = derivative_id as usize;

    let lifted_x = LiftedArray::<T>::lift_for_differentiation(x, derivative_id);

    let result = f(&lifted_x);
    

    // let jacobian = result;
    println!("{:?}", result);
}


