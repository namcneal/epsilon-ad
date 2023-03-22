mod traits; 
mod epsilon_duals;

#[macro_use]
mod differentiation;

use crate::epsilon_duals::*;
use dual_numbers::*;
use differentiation::jacobian::*;
use differentiation::lifting::*;

use ndarray::{array, Array1};


lift_function!(

    fn f<T: Clone>(x: &Array1<T>) -> Array1<T>{ 
        array![x[0].clone(), x[1].clone(), x[2].clone()]
    }

);

fn main() {

    let x = array![1.0, 2.0, 3.0];
    jacobian(f,x);

    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
