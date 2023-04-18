use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::polynomials::*;
use rand::Rng;

fn test_on_D_dim_polynomials<const D: usize>(){
	let num_polynomials   : usize = 10;
	let max_num_monomials : u32   = 5;
	let maximum_power     : u32   = 10;

	let num_eval_points : u32 = 50;
	let iters : [std::ops::Range<u32>; D] = array_init::array_init(|_| 1..num_eval_points);
	
	for iter in iters{
		for _ in iter{

			let mut input : [f64; D] = [0.0; D];
			rand::thread_rng().fill(&mut input[..]);
			let input = EVector::<f64>::from(input.as_slice());

			for i in 0..num_polynomials{
				for N in 1..=max_num_monomials{
					let polynomial = EPolynomial::<f64,D>::random_normal(N as u64, i as u64);
					
					let analytic_result = polynomial.analytic_gradient(&input);
					let epsilon_result  = polynomial.epsilon_gradient(&input.values());

					let non_zero = 1e-16;
					let difference = &analytic_result - &epsilon_result;
					let mut distance = difference.map(|el| *el * *el).sum();
					distance /= D as f64;

					println!("Expected: {}\nReceived: {}\nDifference:{}\n\n", &analytic_result, &epsilon_result, difference);
					assert!(distance < 1e-16)
				}
			}
		}	
	}
}

#[test]
fn polynomials(){
	duplicate::duplicate! {
		[
			dimension;    
			[1];		
			[2];
			[3];
			[4];
			[5];
		]
		test_on_D_dim_polynomials::<dimension>();
	}
}


#[test]
#[ignore]
fn test_polynomials_extra(){
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
		test_on_D_dim_polynomials::<dimension>();
	}
}
