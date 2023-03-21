mod traits; 
mod epsilon_duals;
mod differentiation;

use crate::epsilon_duals::*;
use crate::differentiation::directional_derivative::*;
use perturbation::*;
use dual_numbers::*;
use num_traits;





fn main() {
    let derivative_called : bool        = true; 
    let derivative_id     : *const bool = &derivative_called;
    let derivative_id     : usize       = derivative_id as usize;

    let direction : usize = 1;
    
    let a = 1.0;
    let x1 : DualNumber<f64> = DerivativeInput::lift_for_differentiation(a, derivative_id, direction);
    let x2 = DerivativeInput::lift_for_differentiation(a, derivative_id+1,direction);

    let y = DerivativeInput::lift_for_differentiation(x1.clone(), derivative_id, direction);

    // println!("{:?}", x1*x2);
    println!("{:?}", y*x2);

    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
