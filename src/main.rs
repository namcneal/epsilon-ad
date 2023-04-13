#[macro_use]
mod differentiation;
mod duals;
mod scalar;

use crate::differentiation::lifting::Liftable;
use crate::duals::perturbations::*;
use crate::duals::duals::*;
use crate::duals::dual_arrays::*;
use crate::scalar::Scalar;
use crate::differentiation::lifting::*;
use crate::differentiation::jacobian::*;

use ndarray::{Dim, IxDyn};
use ndarray::{array,Array1,ArrayD, ArrayView1};
use ndarray::{Axis, stack};
use num_traits::Num;

use std::iter::FromIterator;


type Df64 = Dual<f64>;


fn f(x: &DVector<f64>) -> ArrayD<Df64>{ 
    let mut result = Df64::from(0.0);
    for i in 0..x.0.len(){
        result = result + (&x.0[i]* &x.0[i]);
        // println!("{:?}", &x[i]* &x[i]);
    }

    ArrayD::from_elem(IxDyn(&[1]), result)
}

// fn g(x: &mut Array1<Dual<f64>>) -> Array<Dual<f64>>{ 
//     jacobian(f, x)
// }

fn main() {

    let x = array![1.0, 2.0, 3.0, 1.0];
    let mut x = DVector(x.lift(0));

    // let jacobian = jacobian(f,&mut lifted_x).map(|dual| dual.value);
    // println!("{:?}", g(&mut lifted_x));


    let f = DVector::<f64>::graham_schmidt_with_standard_basis;
    // let J = jacobian(f, &mut lifted_x).map(|dual| dual.value);

        // let jacobian = jacobian(f,&mut lifted_x).map(|dual| dual.value);


    println!("{:?}", f(&x).map(|vi| vi.value));

    // println!("{:?}", t);


}

