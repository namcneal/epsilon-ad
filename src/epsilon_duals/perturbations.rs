
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
				&format!("({:?}", self.coefficients[i])
			);
			representation.push_str(
				&format!(" product of εs (id={:?}) ) + ", self.products[i].0)
			);
		}
		write!(f, "{}", &representation)
	}
}

impl <T:Scalar> From<&[NonEmptyEpsilonProduct]> for Perturbation<T>{
	fn from(slice_of_epsilons: &[NonEmptyEpsilonProduct]) -> Self {
		Perturbation { 
			coefficients: SmallVec::from([T::zero(); SVNE]), 
				products: SmallVec::from(slice_of_epsilons)
		}
	}
}

impl <T:Scalar, const N: usize> From<&[NonEmptyEpsilonProduct; N]> for Perturbation<T>{
	fn from(arr_of_epsilons: &[NonEmptyEpsilonProduct; N]) -> Self {
		Perturbation { 
			coefficients: SmallVec::from([T::one(); SVNE]), 
				products: SmallVec::from(arr_of_epsilons.as_slice())
		}
	}
}

impl<T: Scalar> Perturbation<T>{
    pub fn empty_perturbation()->Perturbation<T>{
        Perturbation::<T>{ coefficients: PerturbationData::<T>::new(), 
                           products    : PerturbationData::<NonEmptyEpsilonProduct>::new()}
    }

    pub fn singleton_product(depth:EpsilonFieldType, direction:EpsilonFieldType) -> Perturbation<T>{
        let mut perturbation = Perturbation::<T>::empty_perturbation();
        perturbation.coefficients.push(T::one());
        perturbation.products.push(NonEmptyEpsilonProduct::singleton(depth, direction));

        return perturbation
    }

    pub (crate) fn combine_like_monomials(coefficients:Vec<&T>, products:Vec<&NonEmptyEpsilonProduct>) -> Perturbation<T>{
        let mut coefficients_grouped_by_like_products = HashMap::<NonEmptyEpsilonProduct, Vec<T>>::new();
		for (i, epsilon_product) in products.iter().enumerate(){
            
			match coefficients_grouped_by_like_products.get_mut(&epsilon_product){
				None => {
					coefficients_grouped_by_like_products.insert(**epsilon_product, vec![*coefficients[i]]);
				},
				Some(vector) => {
					vector.push(*coefficients[i]);
				}
			}
		}

		let mut sum_over_like_products  = Vec::<NonEmptyEpsilonProduct>::new();
        let mut sum_over_coefficients_of_like_products = Vec::<T>::new();
		for (unique_product, coefficients) in coefficients_grouped_by_like_products.iter_mut(){
			let summed_coefficient : T = (*coefficients)
				.iter()
				.fold(T::zero(), |acc,x| acc + *x);

			let epsilon_product = NonEmptyEpsilonProduct(unique_product.0);
			sum_over_like_products.push(epsilon_product);
			sum_over_coefficients_of_like_products.push(summed_coefficient);
		}

		Perturbation{coefficients : smallvec::SmallVec::from(sum_over_coefficients_of_like_products), 
					 products     : smallvec::SmallVec::from(sum_over_like_products)}
	}
}
