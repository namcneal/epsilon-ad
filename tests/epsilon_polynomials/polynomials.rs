use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::monomials::*;

#[derive(Debug, Clone)]
pub struct EPolynomial<T: Scalar, const D: usize>(pub (crate) Vec<EMonomial<T,D>>);

impl<T: Scalar, const D: usize> EPolynomial<T,D>{
	pub (crate) fn eval(&self, x:&EVector<T>) -> EReal<T>{
		EReal::<T>::from(
			self.0.iter()
				.map(|mono| mono.eval(x).0)
				.reduce(|acc,item| acc + item)
				.unwrap()[ndarray::Dim(())]
				.clone()
		)
	}

	pub (crate) fn random_normal(num_monomials:u64,seed:u64) -> EPolynomial<T, D>{	
		let mut monomials = Vec::<EMonomial<T,D>>::new();
		for i in 0..num_monomials{
			monomials.push(EMonomial::<T,D>::random_normal(seed+i))
		}

		EPolynomial::<T,D>(monomials)
	}


	pub (crate) fn analytic_gradient(&self, x:&EVector<T>) -> ndarray::Array1<T>{
		let mut gradient = ndarray::Array1::from_elem([D], T::zero());
		for i in 0..D{
			gradient[i] = self.0.iter()
							  .map(|mono| mono.analytic_partial_derivative(i))
							  .map(|pder| pder.eval(x).values())
							  .map(|pder_arr| pder_arr[ndarray::Dim(())])
							  .reduce(|acc,item| acc + item)
							  .unwrap();
		}		

		return gradient;
	}

	pub (crate) fn epsilon_gradient(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = jacobian(|x| self.eval(x), x);
		result.jacobian
	}
}


