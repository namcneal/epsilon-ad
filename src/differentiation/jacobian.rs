use crate::prelude::*;
use crate::differentiation::differentiate::*;

// use crate::epsilon_duals::epsilons::{EpsilonID, NonEmptyEpsilonProduct};

// use std::iter::zip;
// use ndarray::{IxDyn, Array, ArrayD, Array2, s, Dimension};
// use num::Zero;

// #[derive(Debug,Clone)]
// pub struct JacobianResult<T:Scalar,D:ndarray::Dimension>{
//     pub value   :    Array<T,D>,
//     pub jacobian:    EArray<T,ndarray::IxDyn>
// }


pub fn jacobian<'a, T,F,D1,D2>(f: F, x:&ndarray::Array1<T>) -> JacobianResult<T,D2>
where T  : Scalar
//       D1 : ndarray::Dimension,
//       D2 : ndarray::Dimension,
//       F  : Fn(&EArray<T,D1>) -> EArray<T,D2>
{
//     let input_data = EvaluationInput::<T,D1>::from(x.clone());
//     let evaluated_with_epsilons = input_data.tagged_eval(f);
//     // let tags_extracted = evaluated_with_epsilons.extractr

//     // let result_principal_values = result.clone().map(|el| el.value);
//     // let result_infinimal_values = extract_derivative(x.0.len(), &result, derivative_id);
    
//     // JacobianResult{value:    result_principal_values, 
//     //                jacobian: result_infinimal_values}

//     todo!()

}
