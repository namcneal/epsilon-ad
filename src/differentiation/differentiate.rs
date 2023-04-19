use array_init::array_init;

use crate::prelude::*;
use crate::epsilon_duals::epsilons::{Epsilon, NonEmptyEpsilonProduct};
use crate::epsilon_duals::perturbations::Perturbation;

// use std::iter::zip;
// use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
// use num::Zero;
// use std::rc::Rc;


#[derive(Clone, Debug)]
pub (crate) struct EpsilonBasis<const D: usize>([Epsilon; D]);
impl<const D: usize> EpsilonBasis<D>{
	fn new(depth:u16) -> Self{
		let basis_epsilons : [Epsilon; D] = array_init::array_init(|i| Epsilon::singleton(depth, i as u16 + 1));
		EpsilonBasis(basis_epsilons)
	}

	fn uninit() -> Self{
		Self::new(u16::MAX)
	}
}

pub (crate) struct DerivativeInvocation<T,const D: usize, const O: usize> 
	where  T: Scalar,
{ 
	input    : EVector<T>,
	epsilons : [EpsilonBasis<D>; O]
}


type Index = Vec<usize>;
type EpsilonExtractionMap = Vec<(Index, NonEmptyEpsilonProduct)>;
pub (crate) struct DerivativeResult<T,const D: usize, D2,const O: usize> 
	where  T: Scalar,
		   D2: ndarray::Dimension
{ 
	output              : EArray<T,D2>,
	products_to_extract : [EpsilonExtractionMap; O]
}



impl<T, const D: usize,const O: usize> DerivativeInvocation<T,D,O>
    where T: Scalar
{

	pub fn new(input:EVector<T>) -> Self{
		let epsilon_basis_complex = array_init::array_init(|depth| EpsilonBasis::<D>::new(depth as u16));
		DerivativeInvocation{input: input, epsilons:epsilon_basis_complex}
	}

	fn epsilon_products_to_extract(&self) -> [Vec<NonEmptyEpsilonProduct>; O]{
		let epsilon_extraction_maps : [EpsilonExtractionMap; O] = {
			array_init::array_init(|depth| Vec::new())
		};

		

		for i in 0..O{
			match i {
				0 => {
					panic!("This method should not be called for a derivative/depth order of zero. Please check where this call occured.");
				}

				1 => {
					epsilon_extraction_maps[0] = self.epsilons[0].clone().0.iter().enumerate().collect();
				}
				
				k => {
					
					let mut products_to_extract_at_this_depth = Vec::<(Index, NonEmptyEpsilonProduct)>::new();
					for eps in self.epsilons[i].0.iter(){
						for prod in epsilon_products_to_extract[i-1].iter(){
							match (eps * prod).0{
								None =>  (),
								Some(product) => products_to_extract_at_this_depth.push(product)
							}
							
						}
					}

					epsilon_products_to_extract[i] = products_to_extract_at_this_depth;
				}
			}
		}

		return epsilon_products_to_extract
	}

    pub fn tagged_eval<'a,F,D2>(self, f: F) -> DerivativeResult<T,D,D2,O>
		where D2: ndarray::Dimension,
			   F: Fn(&EVector<T>) -> EArray<T,D2>
	{

		let mut x = self.input.clone();
		// Perturb
        for (direction, xi) in x.0.iter_mut().enumerate(){
			for depth in 0..O{
				let perturbation = Perturbation::<T>::from(&[self.epsilons[depth].0[direction]]);
				(*xi).duals = perturbation;
			}
            
        }

		DerivativeResult{output              : f(&x),
						 products_to_extract : self.epsilon_products_to_extract()
		}
	}
}

