use std::array;

use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::rationals::*;
use rand::{Rng, SeedableRng};


fn test_on_D_dim_rationals<const D: usize>(){
	let num_rationals   : usize = 10;
	let max_num_monomials : u32   = 5;
	let maximum_power     : u32   = 3;

	let num_eval_points : u32 = 50;
	let iters : [std::ops::Range<u32>; D] = array_init::array_init(|_| 1..num_eval_points);
	
	for iter in iters{
		for _ in iter{

			let mut input : [f64; D] = array_init::array_init(|_| 0.0);
			rand::rngs::StdRng::seed_from_u64(0).fill(&mut input[..]);
			let input : Vec<f64> = input.iter()
							 .map(|el| f64::from(*el))
							 .collect();

			let input = EVector::<f64>::from(input.as_slice());

			for _ in 0..num_rationals{
				for N in 1..=max_num_monomials{
					let rational = ERational::<f64,D>::random_normal(N as u64);
					// println!("Random input: {:?}\n", &input);
					// println!("Rational function: {:?}\n\n", &rational);
					
					let analytic_result = rational.analytic_hessian(&input);
					let epsilon_result  = rational.epsilon_hessian(&input.values());


					let difference = &analytic_result - &epsilon_result;
					let mut distance = difference.map(|el| *el * *el).sum();
					distance /= f64::from(D as f64);

					let approx_zero = f64::from(1e-16);
					let msg = format!("\n\nExpected: {}\nReceived: {}\nDifference:{}\nDistance:{}\n\n", &analytic_result, &epsilon_result, difference,distance);
					assert!(distance < approx_zero, "{}", &msg);
				}
			}
		}	
	}
}

#[test]
fn rationals(){
	duplicate::duplicate! {
		[
			dimension;    
			[1];		
			[2];
			[3];
			[4];
			[5];
		]
		test_on_D_dim_rationals::<dimension>();
	}
}


#[test]
#[ignore]
fn test_rationals_extra(){
	duplicate::duplicate! {
		[
			dimension; 
			[6];
			[7];
			[8];
			[9];
		    [10];
			[11];
			[12];
			[13];
			[14];
			[15];
			[16];
			[17];
			[18];
			[19];
		]
		test_on_D_dim_rationals::<dimension>();
	}
}
