
#[macro_use]
pub mod scalar;
pub mod epsilon_duals;
pub mod epsilon_arrays;
pub mod ops;
pub mod lifting;
pub mod differentiation;


pub mod prelude {
	pub use crate::scalar::Scalar;
	pub use crate::epsilon_duals::duals::*;
	pub use crate::epsilon_arrays::*;
	pub use crate::lifting::*;
	pub use crate::lifting::Lift;
	pub use crate::differentiation::jacobian::*;
}


// use crate::prelude::*;
// use ndarray::{Dim, IxDyn};
// use ndarray::{array,Array1,ArrayD, ArrayView1};
// use ndarray::{Axis, stack};


// fn f(x: &DVector<f64>) -> ArrayD<Df64>{ 
//     let mut result = Df64::from(0.0);
//     for i in 0..x.0.len(){
//         result = result + (&x.0[i]* &x.0[i]);
//         // println!("{:?}", &x[i]* &x[i]);
//     }

//     ArrayD::from_elem(IxDyn(&[1]), result)
// }

// // fn g(x: &mut Array1<Dual<f64>>) -> Array<Dual<f64>>{ 
// //     jacobian(f, x)
// // }

// fn main() {

//     let x = vec![1.0, 2.0, 3.0, 1.0];
// 	let dim = x.len();
//     let mut x = x.lift(0);

//     // let jacobian = jacobian(f,&mut lifted_x).map(|dual| dual.value);
//     // println!("{:?}", g(&mut lifted_x));


//     let basis = StandardBasis::from(x);
//     let mut R = &*basis.matrix();
//     // let J = jacobian(f, &mut lifted_x).map(|dual| dual.value);

//         // let jacobian = jacobian(f,&mut lifted_x).map(|dual| dual.value);

//     let d = dim;
//     let mut test = EArray(ArrayD::from_elem(IxDyn(&[d,d]), Dual::<f64>::zero()));
//     for i in 0..d{
//         for j in 0..d{
//             for s in 0..d{
//                 test[[i,j]] = &test[[i,j]] +  &(&R[[i,s]] * &R[[j,s]]); 
//             }
//         }
//     }

//     println!("{:?}", test.unlift());

//     // println!("{:?}", t);


// }

