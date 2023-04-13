use crate::duals::duals::Dual;
use crate::duals::perturbations::*;
use crate::duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
// use crate::differentiation::lifting::*;
use crate::duals::dual_arrays::*;
use crate::scalar::Scalar;

use std::iter::zip;
use ndarray::{IxDyn, Array1, ArrayD, Array2, s};
use num::Zero;


pub fn jacobian<'a, T,F>(f: F, x:&mut DVector<T>) -> ArrayD<Dual<T>>
where T  : Scalar+'a,
      F  : Fn(&DVector<T>) -> ArrayD<Dual<T>>
{
    // let _jacobian : ArrayD<T>;

    let derivative_called : bool  = true;
    let derivative_id     : u64   = (&derivative_called as *const bool) as u64;

    for (direction, xi) in x.0.iter_mut().enumerate(){
        let perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
        (*xi).duals = perturbation;
    }

    let result = f(x);

    // return result
    
    extract_derivative(x.0.len(), &result, derivative_id)

}

pub fn extract_derivative<T: Scalar>(input_dimension:usize, with_duals:&ArrayD<Dual<T>>, invocation_id:u64) -> ArrayD<Dual<T>>{
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

    extracted.to_shared().reshape(IxDyn(&jacobian_shape)).to_owned()

}




