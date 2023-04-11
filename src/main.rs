#[macro_use]
mod differentiation;
mod duals;
mod scalar;

use crate::duals::duals::*;
use crate::scalar::Scalar;
use crate::differentiation::jacobian::*;

use ndarray::OwnedRepr;
use ndarray::{array, Array1, ArrayBase};
use num_traits::Num;

use std::iter::FromIterator;


// lift_function!(

//     fn f<T: Clone>(x: &Array1<T>) -> Array1<T>{ 
//         array![&x[0].clone()* &x[1], x[2].clone()]
//     }

// );

// lift_function!(

//     fn g<T: Clone>(x: &Array1<T>) -> Array1<T>{ 
//         Array::from_iter(jacobian(f, x).iter())
//     }

// );


fn f(x: &mut Array1<Dual<f64>>) -> Array1<Dual<f64>>{ 
    array![x[0].clone()*x[1].clone(), x[1].clone(), x[2].clone(), x[3].clone()]
}

fn g(x: &mut Array1<Dual<f64>>) -> Array1<Dual<f64>>{ 
    jacobian(f, x)
}

fn main() {

    let x = array![1.0, 2.0, 3.0, 1.0];
    let mut lifted_x : Array1<Dual<f64>> = x.iter().map(|xi| Dual::<f64>::from(*xi)).collect();

    // println!("{:?}", g(&mut lifted_x));

    println!("{:?}", jacobian(f,&mut lifted_x));
    // let J = forgetful_unlift(&mut jacobian(f,x));

    // println!("{:?}", t);


}


