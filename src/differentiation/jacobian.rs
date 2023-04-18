use crate::prelude::*;
use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
use crate::epsilon_duals::perturbations::*;

use std::iter::zip;
use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
use num::Zero;

#[derive(Debug,Clone)]
pub struct JacobianResult<T:Scalar,D:ndarray::Dimension>{
    pub value   :    Array<T,D>,
    pub jacobian:    EArray<T,ndarray::IxDyn>
}


pub fn jacobian<'a, T,F,D1,D2>(f: F, x:&EArray<T,D1>) -> JacobianResult<T,D2>
where T  : Scalar+'a,
      D1 : ndarray::Dimension,
      D2 : ndarray::Dimension,
      F  : Fn(&EArray<T,D1>) -> EArray<T,D2>
{
    let mut x = x.clone();
    let derivative_called : bool  = true;
    let derivative_id     : u64   = (&derivative_called as *const bool) as u64;

    for (direction, xi) in x.0.iter_mut().enumerate(){
        let perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
        (*xi).duals = perturbation;
    }

    let result = f(&x);

    let result_principal_values = result.clone().map(|el| el.value);
    let result_infinimal_values = extract_derivative(x.0.len(), &result, derivative_id);
    
    JacobianResult{value:    result_principal_values, 
                   jacobian: result_infinimal_values}

}

pub fn extract_derivative<T: Scalar,D:Dimension>(input_dimension:usize, with_duals:&EArray<T,D>, invocation_id:u64) -> EArrayD<T>{
    let original_shape = with_duals.shape();
    
    let mut extracted : Array2<Dual<T>> = ndarray::Array2::<Dual<T>>::zeros((with_duals.len(), input_dimension));

    for (output_dir, output_element_to_consider) in with_duals.iter().enumerate(){
        let duals = &output_element_to_consider.duals;

        for input_dir in 0..input_dimension{
            let reduced_id = EpsilonID(invocation_id, input_dir as u64).reduce();

            for (coeff, eps_prod) in zip(&duals.coefficients, &duals.products){
                let coeff = coeff.clone();

            // If the invocation and the direction are actually there, do something 
                if reduced_id ^ eps_prod.epsilons_within == 0{

                    let mut product_to_add : NonEmptyEpsilonProduct = eps_prod.clone();
                    product_to_add.decrement(true);

                        // If we just removed the last epsilon in the product, we can drop 
                        // it and just add the real portion
                        if product_to_add.num_epsilons == 0{
                            extracted[[output_dir,input_dir]].value += coeff;
                        } else{
                            extracted[[output_dir,input_dir]].duals.coefficients.push(coeff);
                            extracted[[output_dir,input_dir]].duals.products.push(product_to_add);
                        }
                }
            }
        }
    }

    let mut jacobian_shape = original_shape.clone().to_vec();
    jacobian_shape.append(&mut vec![input_dimension]);

    EArray::<T,ndarray::IxDyn>(extracted.to_shared().reshape(IxDyn(&jacobian_shape)).to_owned())

}




