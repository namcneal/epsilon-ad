
use crate::epsilon_duals::epsilons::*;
use crate::scalar::Scalar;
use smallvec::SmallVec;
use num_traits::abs;
use std::collections::HashMap;

const SMALL_VEC_NUMBER_ELEMENTS : usize = 128;
pub (crate) const SVNE : usize = SMALL_VEC_NUMBER_ELEMENTS;
pub (crate) type PerturbationData<T> = SmallVec<[T; SVNE]>;


#[derive(Debug, Clone, PartialEq)]
pub struct Perturbation<T: Scalar>{
    pub coefficients : PerturbationData<T> ,
    pub products     : PerturbationData<NonEmptyEpsilonProduct>
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

    pub fn combine_like_monomials(coefficients:Vec<T>, monomials:Vec<NonEmptyEpsilonProduct>) -> (Vec<T>, Vec<NonEmptyEpsilonProduct>){
		// println!("{:?};\n{:?}\n\n", coefficients, monomials);
        let mut map = HashMap::<(u128,u8), Vec<T>>::new();
		for (i, monomial) in monomials.iter().enumerate(){
            
            let unique_data = (monomial.epsilons_within, monomial.num_epsilons);
			match map.get_mut(&unique_data){
				None => {
					map.insert(unique_data, vec![coefficients[i]]);
				},
				Some(vector) => {
					vector.push(coefficients[i]);
				}
			}
		}

		let epsilon = 1e-16;
		let mut combined_monomials = Vec::<NonEmptyEpsilonProduct>::new();
        let mut combined_coefficients = Vec::<T>::new();
		for (unique_data, coefficients) in map.iter_mut(){
			let summed_coefficient : T = (*coefficients).iter().fold(T::zero(), |acc,x| acc + *x);
			let monomial =NonEmptyEpsilonProduct { epsilons_within: unique_data.0,
                                                                            num_epsilons:   unique_data.1};
			if abs(summed_coefficient) > <T as From<f64>>::from(epsilon){
				combined_monomials.push(monomial);
                combined_coefficients.push(summed_coefficient);
			}
				
		}

		return (combined_coefficients, combined_monomials);
	}
}
