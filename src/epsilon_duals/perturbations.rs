
use crate::epsilon_duals::epsilons::*;
use crate::scalar::Scalar;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::fmt::Debug;

const SMALL_VEC_NUMBER_ELEMENTS : usize = 128;
pub (crate) const SVNE : usize = SMALL_VEC_NUMBER_ELEMENTS;
pub (crate) type PerturbationData<T> = SmallVec<[T; SVNE]>;


#[derive(Clone, PartialEq)]
pub struct Perturbation<T: Scalar>{
    pub coefficients : PerturbationData<T> ,
    pub (crate) products     : PerturbationData<NonEmptyEpsilonProduct>
}

impl<T: Scalar> Debug for Perturbation<T>{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut representation : String = String::new();
		for i in 0..self.products.len(){
			representation.push_str(
				&format!("{:?}", self.coefficients[i])
			);
			representation.push_str(
				&format!("{:?} + ", self.products[i])
			);
		}
		write!(f, "{}", &representation)
	}
}

impl <T:Scalar> From<&[NonEmptyEpsilonProduct]> for Perturbation<T>{
	fn from(slice_of_epsilons: &[NonEmptyEpsilonProduct]) -> Self {
		let coefficients : Vec<T> = std::iter::repeat(T::one())
			.take(slice_of_epsilons.len())
			.collect();

		Perturbation { 
			coefficients: SmallVec::from(coefficients), 
				products: SmallVec::from(slice_of_epsilons)
		}
	}
}

impl <T:Scalar, const N: usize> From<&[NonEmptyEpsilonProduct; N]> for Perturbation<T>{
	fn from(arr_of_epsilons: &[NonEmptyEpsilonProduct; N]) -> Self {
		<Perturbation<T> as From<&[NonEmptyEpsilonProduct]>>::from(arr_of_epsilons.as_slice())
	}
}

impl<T: Scalar> Perturbation<T>{
    pub fn empty_perturbation()->Perturbation<T>{
        Perturbation::<T>{ coefficients: PerturbationData::<T>::new(), 
                           products    : PerturbationData::<NonEmptyEpsilonProduct>::new()}
    }

	pub (crate) fn combine_like_monomials(self) -> Perturbation<T>{
	 Self::combine_like_monomials_iter(self.coefficients.into_iter(), self.products.into_iter())
	}

	pub (crate) fn combine_like_monomials_iter(
			coefficients : impl Iterator<Item = T>, 
			products     : impl Iterator<Item = NonEmptyEpsilonProduct>) -> Perturbation<T>
	{
		let mut coefficients_grouped_by_like_products = HashMap::<NonEmptyEpsilonProduct, Vec<T>>::new();
		for (i, (coefficient, epsilon_product)) in coefficients.zip(products).enumerate(){
            
			match coefficients_grouped_by_like_products.get_mut(&epsilon_product){
				None => {
					coefficients_grouped_by_like_products.insert(epsilon_product, vec![coefficient]);
				},
				Some(vector) => {
					vector.push(coefficient);
				}
			}
		}

		let mut sum_over_like_products  = Vec::<NonEmptyEpsilonProduct>::new();
        let mut sum_over_coefficients_of_like_products = Vec::<T>::new();
		for (unique_product, coefficients) in coefficients_grouped_by_like_products.iter_mut(){
			let summed_coefficient : T = (*coefficients)
				.iter()
				.fold(T::zero(), |acc,x| acc + *x);

			let epsilon_product = NonEmptyEpsilonProduct(unique_product.id());
			sum_over_like_products.push(epsilon_product);
			sum_over_coefficients_of_like_products.push(summed_coefficient);
		}

		Perturbation{coefficients : smallvec::SmallVec::from(sum_over_coefficients_of_like_products), 
					 products     : smallvec::SmallVec::from(sum_over_like_products)}
	}
}
