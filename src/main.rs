mod traits; 
mod epsilon_duals;
mod differentiation;

use crate::epsilon_duals::*;
use dual_numbers::*;
use crate::differentiation::jacobian::*;
use differentiation::lift_for_differentiation::LiftableArray;

use ndarray::{array, Array1};



fn f<T: traits::Scalar>(x:&dyn LiftableArray<T>) -> &dyn LiftableArray<T>{
    array![x[0].clone(), x[1].clone(), x[2].clone()] as &dyn LiftableArray<T>
}


fn main() {
    // let derivative_called : bool        = true; 
    // let derivative_id     : *const bool = &derivative_called;
    // let derivative_id     : usize       = derivative_id as usize;
    
    let value = array![1.0, 1.0, 1.0];

    let result = jacobian(&f, value);
    println!("{:?}", result);

    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
