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

	pub (crate) fn analytic_partial_derivative(&self, i:usize) -> EPolynomial<T,D>{
		EPolynomial::<T,D>(self.0.iter()
							.map(|mono| mono.analytic_partial_derivative(i))
							.collect()
						)
	}


	pub (crate) fn analytic_gradient(&self, x:&EVector<T>) -> EVector<T>{
		let mut gradient = ndarray::Array1::from_elem([D], Dual::<T>::zero());
		for i in 0..D{
			gradient[i] = self.analytic_partial_derivative(i).eval(x).0[ndarray::Dim(())]
		}		
		
		gradient.as_slice().unwrap().into()
	}
	
	pub (crate) fn analytic_hessian(&self, x:&EVector<T>) ->  EMatrix<T>{
		let mut hessian = ndarray::Array2::from_elem([D,D], Dual::<T>::zero());
		for i in 0..D{ for j in 0..D{
			hessian[[i,j]] = self.0.iter()
							  .map(|mono| {
									let pder_i = mono.analytic_partial_derivative(i);
									let pder_ij = pder_i.analytic_partial_derivative(j);
									pder_ij
							  })
							  .map(|pder_ij| pder_ij.eval(x))
							  .map(|pder_arr| pder_arr[ndarray::Dim(())])
							  .reduce(|acc,item| acc + item)
							  .unwrap();
		}}

		EArray::<T, ndarray::Ix2>(hessian)
	}



	pub (crate) fn epsilon_gradient(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = jacobian(|x| self.eval(x), x);
		result.jacobian
	}

	pub (crate) fn epsilon_hessian(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = hessian(|x| self.eval(x), x);
		result.hessian
	}
}


