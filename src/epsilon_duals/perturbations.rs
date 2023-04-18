
use crate::epsilon_duals::epsilons::*;
use crate::scalar::Scalar;
use smallvec::SmallVec;
use num_traits::abs;
use std::collections::HashMap;
use std::fmt::Debug;

const SMALL_VEC_NUMBER_ELEMENTS : usize = 128;
pub (crate) const SVNE : usize = SMALL_VEC_NUMBER_ELEMENTS;
pub (crate) type PerturbationData<T> = SmallVec<[T; SVNE]>;


#[derive(Clone, PartialEq)]
pub struct Perturbation<T: Scalar>{
    pub coefficients : PerturbationData<T> ,
    pub products     : PerturbationData<NonEmptyEpsilonProduct>
}

impl<T: Scalar> Debug for Perturbation<T>{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut representation : String = String::new();
		for i in 0..self.products.len(){
			representation.push_str(
				&format!("({:?}", self.coefficients[i])
			);
			representation.push_str(
				&format!("product of {:?} ϵs ) + ", self.products[i].num_epsilons)
			);
		}

		write!(f, "{}", &representation)
	}
}

impl<T: Scalar> Perturbation<T>{
    pub fn empty_perturbation()->Perturbation<T>{
        Perturbation::<T>{ coefficients: PerturbationData::<T>::new(), 
                           products    : PerturbationData::<NonEmptyEpsilonProduct>::new()}
    }

    pub fn singleton_product(invocation_id:u64, direction:u64) -> Perturbation<T>{
        let mut perturbation = Perturbation::<T>::empty_perturbation();
        perturbation.coefficients.push(T::one());
        perturbation.products.push(NonEmptyEpsilonProduct::singleton(invocation_id, direction));

        return perturbation
    }

    pub fn combine_like_monomials(coefficients:Vec<T>, monomials:Vec<NonEmptyEpsilonProduct>) -> Perturbation<T>{
		// println!("{:?};\n{:?}\n\n", coefficients, monomials);
        let mut map = HashMap::<(AggregateID,EpsilonCount), Vec<T>>::new();
		for (i, monomial) in monomials.iter().enumerate(){
            
            let next_monomial_to_add_deconstructed = (monomial.epsilons_within, monomial.num_epsilons);
			match map.get_mut(&next_monomial_to_add_deconstructed){
				None => {
					map.insert(next_monomial_to_add_deconstructed, vec![coefficients[i]]);
				},
				Some(vector) => {
					vector.push(coefficients[i]);
				}
			}
		}

		let epsilon = 0.0;
		let mut combined_monomials = Vec::<NonEmptyEpsilonProduct>::new();
        let mut combined_coefficients = Vec::<T>::new();
		for (unique_data, coefficients) in map.iter_mut(){
			let summed_coefficient : T = (*coefficients).iter()
				.fold(T::zero(), |acc,x| acc + *x);

			let epsilon_product =NonEmptyEpsilonProduct { epsilons_within: unique_data.0,
                                                                            	  num_epsilons:   unique_data.1};
			if abs(summed_coefficient) > <T as From<f64>>::from(epsilon){
				combined_monomials.push(epsilon_product);
                combined_coefficients.push(summed_coefficient);
			}
				
		}

		Perturbation{coefficients : smallvec::SmallVec::from(combined_coefficients), 
					 products     : smallvec::SmallVec::from(combined_monomials)}
	}
}
