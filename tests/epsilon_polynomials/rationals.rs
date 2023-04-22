use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::monomials::*;
use crate::epsilon_polynomials::polynomials::*;

#[derive(Debug, Clone)]
pub struct ERational<T: Scalar, const D: usize>{
	pub (crate) numerator   :  EPolynomial<T,D>,
	pub (crate) denominator : EPolynomial<T,D>
}

impl<T: Scalar, const D: usize> ERational<T,D>{
	pub (crate) fn eval(&self, x:&EVector<T>) -> EReal<T>{

		let result = &self.numerator.eval(x).clone() / &self.denominator.eval(x).clone();
		// println!("Result of ERational evluation: {:?}", &result);
		return result;
	}

	pub (crate) fn random_normal(num_monomials:u64) -> ERational<T, D>{	
		ERational::<T,D>{numerator: EPolynomial::<T,D>::random_normal(num_monomials,0), 
					     denominator: EPolynomial::<T,D>::random_normal(num_monomials,100)
		}
	}

	pub (crate) fn analytic_partial_derivative(&self, x:&EVector<T>, i:usize) -> T{
		let high = self.numerator;
		let d_high = self.numerator.analytic_partial_derivative(i);

		let low  = self.denominator.eval(x);
		let d_low  = self.denominator.analytic_partial_derivative(i);

		let new_numerator = d_high * low  -  d_low *high;
		let new_denominator = low*low;

		new_numerator / new_denominator
		// println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", high, low, d_high, d_low,result);
	}


	pub (crate) fn analytic_gradient(&self, x:&EVector<T>) -> ndarray::Array1<T>{
		ndarray::arr1(
			(0..D).into_iter().map(|i| self.analytic_partial_derivative(x, i)).collect::<Vec<T>>().as_slice()
		)
	}

	pub (crate) fn analytic_hessian(&self, x:&EVector<T>) -> ndarray::Array2<T>{
		let D = x.len();
		let mut hessian = ndarray::Array2::from_elem([D,D], T::zero());
		for i in 0..D{ 
			let partial_i = self.analytic_partial_derivative(x, i);
			
			for j in 0..D{
				let partial_ij = partial_i.analytic_partial_derivative(x,i);
			
				hessian[[i,j]] =  
		}}

		return hessian
		
	}

	pub (crate) fn epsilon_gradient(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = jacobian(|x| self.eval(x), &x);
		
		// println!("Jacobian: {:?}\n---------------------\n", result.jacobian);

		result.jacobian
	}
}


