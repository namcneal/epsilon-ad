// mod epsilon_polynomials;

// mod jacobian_testing;

// mod higher_order_derivatives;


use epsilon_ad::prelude::*;
use epsilon_ad::differentiation::*;

fn Ef(x:&EVector<f64>) -> EMatrix<f64>{
    let d = x.len();
    let mut outer = ndarray::Array2::from_elem(ndarray::Dim([d,d]), Dual::<f64>::zero());
    
    for a in 0..d{
        for b in 0..d{
            outer[[a,b]] = &x[a] * &x[b];
        }
    }

    return EArray(outer)
}

use ndarray::{s};

#[test]
fn main(){
    const dim : usize = 3;
    const order : usize = 2;
    let x0 = ndarray::arr1(&[3.0, 5.0,7.0]);
    let derivative_call = DerivativeInvocation::<f64,order>::new(x0);

    let output = derivative_call.tagged_eval(&Ef);
    
    println!("{:?}", output.extract_all_derivatives());
    // todo!()
}