use crate::duals::duals::Dual;
use crate::duals::perturbations::*;
use crate::duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
// use crate::differentiation::lifting::*;
use crate::scalar::Scalar;

use std::iter::zip;
use ndarray::{Array1, ArrayD, Array2};
use num::Zero;


pub fn jacobian<'a, T,F>(f: F, x:&mut Array1<Dual<T>>) -> Array1<Dual<T>>
where T  : Scalar+'a,
      F  : Fn(&mut Array1<Dual<T>>) -> Array1<Dual<T>>
{
    // let _jacobian : ArrayD<T>;

    let derivative_called : bool  = true;
    let derivative_id     : u64   = (&derivative_called as *const bool) as u64;

    for (direction, xi) in x.iter_mut().enumerate(){
        let perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
        (*xi).duals = perturbation;
    }

    let result = f(x);

    return result

    // let value : Array1<T> = result.iter().map(|dual| dual.value).collect();
    
    // extract_derivative(x.len(), &result, derivative_id)

}

pub fn extract_derivative<T: Scalar>(input_dimension:usize, with_duals:&Array1<Dual<T>>, invocation_id:u64) -> Array2<Dual<T>>{
    let mut extracted : Array2<Dual<T>> = ndarray::Array2::<Dual<T>>::zeros((with_duals.len(), input_dimension));

    for (i, output_element_to_consider) in with_duals.iter().enumerate(){
        let duals = &output_element_to_consider.duals;

        for d in 0..input_dimension{
            let reduced_id = EpsilonID(invocation_id, d as u64).reduce();

            for (coeff, eps_prod) in zip(&duals.coefficients, &duals.products){
                let coeff = coeff.clone();

            // If the invocation and the direction are actually there, do something 
                if reduced_id ^ eps_prod.epsilons_within == 0{

                    let mut product_to_add : NonEmptyEpsilonProduct = eps_prod.clone();
                    product_to_add.decrement(true);

                        // If we just removed the last epsilon in the product, we can drop 
                        // it and just add the real portion
                        if product_to_add.num_epsilons == 0{
                            extracted[[i,d]].value += coeff;
                        } else{
                            extracted[[i,d]].duals.coefficients.push(coeff);
                            extracted[[i,d]].duals.products.push(product_to_add);
                        }
                }
            }
        }
    }

    
    return extracted;

}




