use array_init::array_init;
use ndarray::Dimension;

use crate::prelude::*;
use crate::epsilon_duals::epsilons::{Epsilon, NonEmptyEpsilonProduct, EpsilonProduct};
use crate::epsilon_duals::perturbations::Perturbation;

use itertools::{Itertools, iterate};
use std::collections::HashMap;
use ndarray::s;

use smallvec::SmallVec;


// use std::iter::zip;
// use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
// use num::Zero;
// use std::rc::Rc;

const ASSUMED_MAXIMUM_NUM_DERIVATIVES : usize = 16;

#[derive(Clone, Debug)]
pub struct EpsilonBasis(SmallVec::<[Epsilon; ASSUMED_MAXIMUM_NUM_DERIVATIVES]>);
impl EpsilonBasis{
	fn new(depth:u16) -> Self{
		let mut basis_epsilons = SmallVec::<[Epsilon; ASSUMED_MAXIMUM_NUM_DERIVATIVES]>::new();
		for i in 0..depth{
			basis_epsilons[i as usize] = Epsilon::singleton(depth, i as u16 + 1);
		}

		EpsilonBasis(basis_epsilons)
	}

	fn uninit() -> Self{
		Self::new(u16::MAX)
	}
}

pub struct DerivativeInvocation<T, const K: usize> 
	where  T: Scalar,
{ 
	dimension : usize,
	input     : EVector<T>,
	pub epsilons : [EpsilonBasis; K]
}

type Index = Vec<usize>;
fn index2slice(index:&Vec<usize>) -> Vec<ndarray::Slice>{
	index.iter().map( |idx|
		ndarray::Slice::new(*idx as isize, Some(*idx as isize + 1), 1)
	).collect()
}

#[derive(Debug)]
struct DerivativeTensorIndex{
	order : usize, 
	index : Index
}

type EpsilonExtractionMap = HashMap<NonEmptyEpsilonProduct, Vec<DerivativeTensorIndex>>;
#
[derive(Debug)]
pub struct DerivativeResult<T, D2, const K: usize> 
	where  T: Scalar,
		   D2: ndarray::Dimension
{ 
	input_dimension     : usize,
	output              : EArray<T,D2>,
	each_depth_extraction_map : EpsilonExtractionMap
}



impl<T, const K: usize> DerivativeInvocation<T,K>
    where T: Scalar
{

	pub fn eval<'a,F,D>(self, f: F) -> DerivativeResult<T,D,K>
		where D: ndarray::Dimension,
			  F: Fn(&EVector<T>) -> EArray<T,D>
	{
		self.tagged_eval(f)
	}

	pub fn call<'a,F,D>(self, f: F) -> DerivativeResult<T,D,K>
	where D: ndarray::Dimension,
		  F: Fn(&EVector<T>) -> EArray<T,D>
{
	self.tagged_eval(f)
}

	fn input_shape(&self)      -> &[usize] { self.input.0.shape() }
	fn derivative_order(&self) ->   usize  { K+1 }

	pub fn new(input:ndarray::Array1<T>) -> Self{
		let epsilon_basis_complex = array_init::array_init(|depth| EpsilonBasis::new(depth as u16));
		DerivativeInvocation{dimension: input.len(), input: input.lift(), epsilons:epsilon_basis_complex}
	}

	fn epsilon_products_to_extract(&self) -> EpsilonExtractionMap{
		let mut map : EpsilonExtractionMap = HashMap::new();

		for j in 0..K{
			let derivative_order = j+1;
			let combinations = (0..self.dimension).combinations(derivative_order);
					
			for derivative_combination in combinations{
				// println!("{:?}", derivative_combination);
				let corresponding_epsilon_product = derivative_combination.iter()
					.enumerate()
					.map(|(depth,dir)| self.epsilons[depth].0[*dir])
					.map(|nonempty| EpsilonProduct::from(nonempty))
					.reduce(|acc,item| acc * &item)
					.unwrap();

				match corresponding_epsilon_product.0{
					None => panic!("This should never be none. Something in the epsilon product went wrong."),
					Some(epsilon_product) => {
						let all_indices_the_product_goes_to = derivative_combination.into_iter()
							.permutations(derivative_order)
							.map(|permuted_index| DerivativeTensorIndex{ order : derivative_order, index: permuted_index})
							.collect();

						map.insert(epsilon_product, all_indices_the_product_goes_to);
					}
				}
			}
		}

		return map;
	}

    pub fn tagged_eval<'a,F,D>(self, f: F) -> DerivativeResult<T,D,K>
		where D: ndarray::Dimension,
			  F: Fn(&EVector<T>) -> EArray<T,D>
	{

		let mut x = self.input.clone();
		// Perturb
        for (direction, xi) in x.0.iter_mut().enumerate(){
			for depth in 0..K{
				let perturbation = Perturbation::<T>::from(&[self.epsilons[depth].0[direction]]);
				(*xi).duals = perturbation;
			}
            
        }

		DerivativeResult{ input_dimension     : x.len(),
						  output              : f(&x),
						  each_depth_extraction_map : self.epsilon_products_to_extract()
		}
	}
}

impl<T, D, const K: usize> DerivativeResult<T,D,K>
	where T: Scalar,
		D: Dimension
{
	
	fn input_shape(&self)      ->   usize  { self.input_dimension }
	fn output_shape(&self)     -> &[usize] { self.output.0.shape() }
	fn derivative_order(&self) ->   usize  { K+1 }

	fn derivative_shape(&self, input_shape: usize, output_shape:&[usize], derivative_order:usize) -> Vec<usize>{
		assert!(derivative_order > 0);
		let derivative_indices = std::iter::repeat(input_shape).take(derivative_order);
		let mut final_shape = output_shape.to_vec();
		final_shape.extend(derivative_indices);
		
		return final_shape
	}

	fn empty_derivative_tensors(&self) -> Vec::<ndarray::ArrayD<T>>{
		let output_shape = self.output_shape();
		
		let mut tensors = Vec::<ndarray::ArrayD<T>>::new();
		for j in 0..K{
			let derivative_depth = j+1;
			let derivative_shape = self.derivative_shape(self.input_shape(), self.output_shape(), derivative_depth);
			let derivative_tensor = ndarray::Array::from_elem(derivative_shape, T::zero());
			tensors.push(derivative_tensor);
		}

		return tensors;
	}

	pub fn extract_all_derivatives(&self) -> (ndarray::Array<T,D>, Vec<ndarray::ArrayD<T>>){
		let mut derivatives = self.empty_derivative_tensors();

		let all_output_indices = self.output_shape().iter().map(|axis_size| 0..*axis_size);

		for output_index in all_output_indices.multi_cartesian_product(){
			let output_index_slice = index2slice(&output_index);

			let current_output_dual_element = self.output.slice_each_axis( |axis_description|{ 
				let axis_idx = axis_description.axis.0;
				output_index_slice[axis_idx]
			});

			assert!(current_output_dual_element.len() == 1);
			let perturbation = &current_output_dual_element.first().unwrap().duals;
			for (i,epsilon_product) in perturbation.products.iter().enumerate(){
				match self.each_depth_extraction_map.get(epsilon_product){
					None => panic!("This epsilon has escaped the table that maps it to its output tensor indices. This should not happen."),
					
					Some(tensor_indices) => {
						for tensor_index in tensor_indices{
							let order = tensor_index.order;

							let mut all_indices_of_tensor = output_index_slice.clone();

							let tensor_derivative_index_as_slice = index2slice(&tensor_index.index);
							all_indices_of_tensor.extend(tensor_derivative_index_as_slice);
							
							let mut derivative_element = derivatives[order-1].slice_each_axis_mut(|axis_descr| all_indices_of_tensor[axis_descr.axis.0]);
							derivative_element += &ndarray::arr0(perturbation.coefficients[i]);
						}
					}
				}
			}
		}

		return (self.output.values(), derivatives)
	}
	

}