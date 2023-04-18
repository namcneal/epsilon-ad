use crate::prelude::*;
use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};
use crate::epsilon_duals::perturbations::*;

use std::iter::zip;
use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
use num::Zero;
use std::rc::Rc;

pub (crate) struct EvaluationInvocation;

pub (crate)struct EvaluationInput<T: Scalar, D:ndarray::Dimension>{
    input : EArray<T,D>,
    invocations : Vec<Rc<EvaluationInvocation>>
}

pub (crate)struct EvaluationOutput<T: Scalar, D:ndarray::Dimension>{
    output : EArray<T,D>,
    invocations : Vec<Rc<EvaluationInvocation>>
}


impl<T: Scalar, D: ndarray::Dimension> From<Array<T,D>> for EvaluationInput<T,D>{
    fn from(value: Array<T,D>) -> Self {
        EvaluationInput { 
            input: value.lift(), 
            invocations : Vec::<Rc<EvaluationInvocation>>::new()
         }
    }
}

impl<T,D> From<EvaluationOutput<T,D>> for EvaluationInput<T,D>
    where T: Scalar, 
          D: ndarray::Dimension,
{
    fn from(value: EvaluationOutput<T,D>) -> Self {
        EvaluationInput { 
            input: value.output,
            invocations : value.invocations
         }
    }
}


impl<T,D> EvaluationInput<T,D>
    where T: Scalar, D: ndarray::Dimension
{
    pub fn tagged_eval<'a,F,D2>(&self, f: F) -> EvaluationOutput<T,D2>
        where T  : Scalar+'a,
            D2 : ndarray::Dimension,
            F  : Fn(&EArray<T,D>) -> EArray<T,D2>
    {
        // Tag 
        let mut x = self.input.clone();
        let derivative_called = EvaluationInvocation;
        let derivative_id     : u64   = (&derivative_called as *const EvaluationInvocation) as u64;

        // Perturb
        for (direction, xi) in x.0.iter_mut().enumerate(){
            let perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
            (*xi).duals = perturbation;
        }

        // Evaluate
        let result = f(&x);

        EvaluationOutput{
            output : result,
            invocations: [self.invocations, vec![Rc::new(derivative_called)]].concat()
        }
    }

    pub fn extract_products_with_tag(input_dimension:usize, output:EvaluationOutput<T,D>, invocation_id:u64) -> EvaluationOutput<T,ndarray::IxDyn>
        where T: Scalar,
            D:Dimension
    {

    let with_duals = output.output;
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

    let mut new_shape = original_shape.clone().to_vec();
    new_shape.append(&mut vec![input_dimension]);

    let new_output = EArray::<T,ndarray::IxDyn>(extracted.to_shared().reshape(IxDyn(&new_shape)).to_owned());
    
    EvaluationOutput { output: new_output,
                       invocations: output.invocations
                    }

}




}

