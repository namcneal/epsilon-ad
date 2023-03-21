mod traits; 
mod epsilon_duals;
mod differentiation;

use crate::epsilon_duals::*;
use crate::differentiation::jacobian::*;

use ndarray::{array, Array1};



fn f(x:Array1<f64>) -> Array1<f64>{
    array![x[0], x[1], x[2]]
}


fn main() {
    // let derivative_called : bool        = true; 
    // let derivative_id     : *const bool = &derivative_called;
    // let derivative_id     : usize       = derivative_id as usize;
    
    let value = array![1.0, 1.0, 1.0];

    let result = jacobian::<f64>(f, value);
    println!("{:?}", result);

    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
