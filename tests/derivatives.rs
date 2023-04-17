use epsilon_ad::prelude::*;
use num::{range, pow}; 

#[test]
fn test_square(){
	fn f(x:f64) -> f64{
		x*x
	}

	fn Ef(x:&Ef64) -> Ef64{
		x * x
	}

	let dx = 0.1;
	let xs = range(0,100).map(|x| x as f64 / dx);
	let ys  : Vec<f64> = xs.clone().map(|x| f(x as f64)).collect();
	let dys : Vec<f64> = xs.clone().map(|x| 2.0* x as f64).collect();
	let eys : Vec<JacobianResult<f64,ndarray::Ix0>>= xs.clone().map(|x| jacobian(Ef, &ndarray::arr0(x as f64))).collect();

	for (i,y) in ys.iter().enumerate(){
		let expected_value = *y;
		let received_value = eys[i].value[ndarray::Dim(())];
		assert!(pow(expected_value - received_value,2) <= 1e-16);
		
		let expected_deriv = dys[i];
		let received_deriv = eys[i].jacobian[[0]].value;
		assert!(pow(expected_deriv - received_deriv, 2) <= 1e-16);	
	}
}

#[test]
fn test_pow(){
	fn f(x:f64,k:usize) -> f64{
		pow(x,k)
	}

	fn Ef(x:&Ef64, k:usize) -> Ef64{
		x.pow(k)
	}

	for k in 1..10{
		let dx = 0.01;	
		let xs = range(1,100).map(|x| x as f64 * dx);
		
		let ys  : Vec<f64> = xs.clone().map(|x| f(x as f64,k)).collect();
		let dys : Vec<f64> = xs.clone().map(|x| (k as f64) * pow(x as f64, k-1)).collect();
		
		let eys : Vec<JacobianResult<f64,ndarray::Ix0>>= xs.clone().map(|x| jacobian(|x| Ef(x,k), &ndarray::arr0(x as f64))).collect();

		for (i,y) in ys.iter().enumerate(){
			let x = xs.clone().collect::<Vec<f64>>()[i];
			
			let expected_value = *y;
			let received_value = eys[i].value[ndarray::Dim(())];
			
			let mut err_msg = format!("Error computing the {}-times power for {}\n", k,x);
			err_msg.push_str(&format!("Expected {:?}, but received {:?}", &expected_value, &received_value));
			assert!(pow(expected_value - received_value,k) <= 1e-16, "{}", &err_msg);
				
			let expected_deriv = dys[i];
			let received_deriv = eys[i].jacobian[[0]].value;
			
			let mut err_msg = format!("Error computing the {}-times power derivative for {}\n", k,x);
			err_msg.push_str(&format!("Expected {:?}, but received {:?}", &expected_deriv, &received_deriv));

			assert!(pow(expected_deriv - received_deriv, k) <= 1e-16, "{}", err_msg);	
		}
	}
}
 

mod epsilon_polynomials;
use epsilon_polynomials::monomials::*;

#[test]
fn test_on_random_monomials1(){
	let num_monomials:usize = 500;
	let maximum_power:u32   = 20;
	
	for i in 1..100{
		let input = EVector::from_vec(vec![Dual::from(i as f64)]);
		for _ in 0..num_monomials{
			let monomial = EMonomial::<f64,1>::random_normal();
			
			let analytic_result = monomial.analytic_gradient(&input);
			let epsilon_result  = monomial.epsilon_gradient(&input.values());

			let ratio = &analytic_result / &epsilon_result;
			let distance = ratio.map(|el| pow(*el-1.0,2)).sum();

			println!("Expected: {}\n,Received: {}, Ratio:{}", &analytic_result, &epsilon_result, ratio);
			assert!(distance < 1e-16)
		}	
	}
}
