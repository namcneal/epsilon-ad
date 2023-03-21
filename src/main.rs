mod traits; 
mod epsilon_duals;
mod differentiation;

use crate::epsilon_duals::*;
use dual_numbers::*;
use crate::differentiation::jacobian::*;

use ndarray::{array, Array1};



fn f<T: Clone>(x:Array1<T>) -> Array1<T>{
    array![x[0].clone(), x[1].clone(), x[2].clone()]
}


fn main() {
    // let derivative_called : bool        = true; 
    // let derivative_id     : *const bool = &derivative_called;
    // let derivative_id     : usize       = derivative_id as usize;
    
    let value = array![1.0, 1.0, 1.0];

    let result = jacobian::<f64>(f::<DualNumber<f64>>, value);
    println!("{:?}", result);

    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
