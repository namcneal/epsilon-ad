// use epsilon_ad::prelude::*;

// #[test]
// fn hessian_of_cube(){
//     fn f(x:f64) -> f64{
// 		x*x*x
// 	}

// 	fn epsilon_f(x:&Ef64) -> Ef64{
// 		x*x*x.clone()
// 	}

//     let x0 = ndarray::arr0(1.0);
//     hessian(|x| epsilon_f(x), &x0.lift());
// }   