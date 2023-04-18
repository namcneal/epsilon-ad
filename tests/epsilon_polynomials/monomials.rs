use epsilon_ad::prelude::*;
use epsilon_ad::differentiation::lifting;

use rand::prelude::*;
use rand_distr::Normal;
use rand::seq::IteratorRandom;

#[derive(Debug, Clone)]
pub (crate) struct EMonomial<T: Scalar, const D: usize>{
	coefficient: T,
	exponents  : [u32; D],
}

impl<T: Scalar, const D: usize> EMonomial<T,D>{
	pub (crate) fn eval(&self, x:&EVector<T>) -> EReal<T>{
		EReal::<T>::from(
			self.exponents.iter().zip((*x).iter())
				.map(|(exp,base)| base.pow(*exp as usize))
				.fold(Dual::<T>::from(self.coefficient), |acc: Dual<T>, item| acc * item)
		)
	}

	pub (crate) fn random_normal(seed:u64) -> EMonomial<T, D>{	
		let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
		
		let normal = Normal::new(0.0, 1.0).unwrap();
		let coefficient= <T as From<f64>>::from(normal.sample(&mut rng));

		const MAX_VARIABLE_POWER : u32 = 15;
		let mut exponents : [u32; D] = [0; D];
		for var in 0..D{
			let power    : u32 = rng.gen_range(1..=MAX_VARIABLE_POWER);
			exponents[var] = power ;
		}
		
		EMonomial::<T,D>{
			coefficient : coefficient,
			exponents   : exponents
		}
	}

	pub (crate) fn analytic_partial_derivative(&self, direction:usize) -> EMonomial<T,D>{
		let mut derivative_coefficient = self.coefficient;
		let mut derivative_exponents   = self.exponents.clone();
		
		if self.exponents[direction] == 0{
			derivative_coefficient = T::zero();
			derivative_exponents   = [0; D];
		} else {
			derivative_coefficient *= <T as From<f64>>::from(self.exponents[direction] as f64);
			derivative_exponents[direction] -= 1;
		}

		EMonomial::<T,D>{coefficient: derivative_coefficient,
						 exponents:   derivative_exponents} 
	}

	pub (crate) fn analytic_gradient(&self, x:&EVector<T>) -> ndarray::Array1<T>{
		let mut gradient = ndarray::Array1::from_elem([D], T::zero());
		for i in 0..D{
			let partial_derivative = self.analytic_partial_derivative(i);
			gradient[i] = partial_derivative.eval(x).values()[ndarray::Dim(())]
		}		

		return gradient;
	}

	pub (crate) fn epsilon_gradient(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = jacobian(|x| self.eval(x), x);
		result.jacobian.values()
	}
}