use array_init::array_init;
use ndarray::Dimension;

use crate::prelude::*;
use crate::epsilon_duals::epsilons::{Epsilon, NonEmptyEpsilonProduct, EpsilonProduct};
use crate::epsilon_duals::perturbations::Perturbation;

use itertools::{Itertools, iterate};
use std::collections::HashMap;
use ndarray::s;

use smallvec::SmallVec;

#[derive(Debug)]
struct DerivativeOrder(u16);
impl DerivativeOrder{
	fn new(i:u16) -> Self{
		assert!(i > 0);
		DerivativeOrder(i)
	}
	
	fn as_index(&self) -> usize{
		(self.0 - 1) as usize
	}

	fn for_epsilon(&self) -> u16{
		self.0
	}
}


// use std::iter::zip;
// use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
// use num::Zero;
// use std::rc::Rc;

const ASSUMED_MAXIMUM_NUM_DERIVATIVES : usize = 16;

#[derive(Clone, Debug)]
pub struct EpsilonBasis(SmallVec::<[Epsilon; ASSUMED_MAXIMUM_NUM_DERIVATIVES]>);
impl EpsilonBasis{
	fn new(input_dim:usize, order:DerivativeOrder) -> Self{
		let mut basis_epsilons = SmallVec::<[Epsilon; ASSUMED_MAXIMUM_NUM_DERIVATIVES]>::new();
		
		for dir in 1..=input_dim{
			basis_epsilons.push( Epsilon::singleton(order.for_epsilon(), dir as u16) );
		}
		
		EpsilonBasis(basis_epsilons)
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
	order : DerivativeOrder, 
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
	each_order_extraction_map : EpsilonExtractionMap
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
		let dim = input.len();
		let epsilon_basis_complex = array_init::array_init(|idx| EpsilonBasis::new(dim, DerivativeOrder::new(idx as u16 + 1)));
		DerivativeInvocation{dimension: input.len(), input: input.lift(), epsilons:epsilon_basis_complex}
	}

	fn epsilon_products_to_extract(&self) -> EpsilonExtractionMap{
		let mut map : EpsilonExtractionMap = HashMap::new();

		for order in 1..=K{
			let combinations = (0..self.dimension).combinations(order);
					
			for derivative_combination in combinations{
				// println!("{:?}", derivative_combination);
				let corresponding_epsilon_product = derivative_combination.iter()
					.enumerate()
					.map(|(idx,dir)| self.epsilons[idx].0[*dir])
					.map(|nonempty| EpsilonProduct::from(nonempty))
					.reduce(|acc,item| acc * &item)
					.unwrap();

				match corresponding_epsilon_product.0{
					None => panic!("This should never be none. Something in the epsilon product went wrong."),
					Some(epsilon_product) => {
						let all_indices_the_product_goes_to = derivative_combination.into_iter()
							.permutations(order)
							.map(|permuted_index| DerivativeTensorIndex{ order : DerivativeOrder::new(order as u16), index: permuted_index})
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
			for epsilon_order in self.epsilons.iter(){
				let perturbation = Perturbation::<T>::from(&[epsilon_order.0[direction]]);
				(*xi).duals = perturbation;
			}
            
        }

		DerivativeResult{ input_dimension     : x.len(),
						  output              : f(&x),
						  each_order_extraction_map : self.epsilon_products_to_extract()
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
		let mut tensors = Vec::<ndarray::ArrayD<T>>::new();
		for order in 1..=K{
			let derivative_shape = self.derivative_shape(self.input_shape(), self.output_shape(), order);
			let derivative_tensor = ndarray::Array::from_elem(derivative_shape, T::zero());
			tensors.push(derivative_tensor);
		}

		return tensors;
	}

	pub fn extract_all_derivatives(&self) -> (ndarray::Array<T,D>, Vec<ndarray::ArrayD<T>>){
		let mut derivatives = self.empty_derivative_tensors();
		
		let all_output_indices : Vec<std::ops::Range<usize>> = {
			match self.output_shape(){
				[] => vec![0..1],
				_ => self.output_shape().iter().map(|axis_size| 0..*axis_size).collect()
			}
		};
		
		for output_index in all_output_indices.into_iter().multi_cartesian_product(){
			let output_index_slice = index2slice(&output_index);

			let current_output_dual_element = self.output.slice_each_axis( |axis_description|{ 
				let axis_idx = axis_description.axis.0;
				output_index_slice[axis_idx]
			});

			assert!(current_output_dual_element.len() == 1);
			let perturbation = &current_output_dual_element.first().unwrap().duals;
			for (i,epsilon_product) in perturbation.products.iter().enumerate(){
				match self.each_order_extraction_map.get(epsilon_product){
					None => panic!("This epsilon has escaped the table that maps it to its output tensor indices. This should not happen."),
					
					Some(tensor_indices) => {

						for tensor_index in tensor_indices{

							let mut all_indices_of_tensor = output_index_slice.clone();

							let tensor_derivative_index_as_slice = index2slice(&tensor_index.index);
							all_indices_of_tensor.extend(tensor_derivative_index_as_slice);
							
							let mut derivative_element = derivatives[tensor_index.order.as_index()].slice_each_axis_mut(|axis_descr| all_indices_of_tensor[axis_descr.axis.0]);
							println!("{:?}", &ndarray::arr0(perturbation.coefficients[i]));
							derivative_element += &ndarray::arr0(perturbation.coefficients[i]);
						}
					}
				}
			}
		}

		return (self.output.values(), derivatives)
	}
	

}