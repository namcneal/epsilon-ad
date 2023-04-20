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


	pub (crate) fn analytic_gradient(&self, x:&EVector<T>) -> ndarray::Array1<T>{
		let high = (self.numerator.eval(x))[ndarray::Dim(())].value;
		let d_high = self.numerator.analytic_gradient(x);

		let low  = (self.denominator.eval(x))[ndarray::Dim(())].value;
		let d_low  = self.denominator.analytic_gradient(x);

		let new_numerator = d_high.map(|el| low * *el)  -  d_low.map(|el| high * *el);
		let new_denominator = low*low;

		let result = new_numerator.mapv(|el| el / new_denominator);
		// println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", high, low, d_high, d_low,result);

		return result;
	}

	pub (crate) fn epsilon_gradient(&self, x:&ndarray::Array1<T>) -> ndarray::ArrayD<T>{
		let result = jacobian(|x| self.eval(x), &x);
		
		// println!("Jacobian: {:?}\n---------------------\n", result.jacobian);

		result.jacobian
	}
}


