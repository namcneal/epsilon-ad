
use std::marker::PhantomData;

use crate::prelude::*;

struct EMonomial<T: Scalar, const D: usize>{
	coefficient: T,
	exponents  : [u32; D],
}



impl<T: Scalar, const D: usize> EMonomial<T,D>{
	fn eval(&self, x:&EVector<T>) -> EReal<T>{
		EArray::<T,ndarray::Ix0>( 
			ndarray::arr0(
				self.exponents.iter().zip((*x).iter())
					.map(|(exp,base)| base.pow(*exp as usize))
					.fold(Dual::<T>::from(self.coefficient), |acc: Dual<T>, item| acc * item)
		))
	}
}