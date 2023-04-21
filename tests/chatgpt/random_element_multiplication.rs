use ndarray::{Array, ArrayView1};
use epsilon_ad::prelude::*;
use ndarray_linalg::{Norm, Scalar};
use num_traits::abs;

fn create_rational_function<S: epsilon_ad::Scalar + Copy>(seed: u64, array1: ArrayView1<S>) -> S {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let n = array1.len();
    let mut numerator = S::zero();
    let mut denominator = S::one();
    for i in 0..n {
        let coeff = <S as From<f64>>::from(rng.gen()); // generate a random coefficient
        numerator = numerator + coeff * array1[[i]];
        denominator = denominator * (S::one() + array1[[i]]);
    }
    numerator / denominator
}

fn epsilon_create_rational_function<S: epsilon_ad::Scalar + Copy>(seed: u64, array1: &EVector<S>) -> EReal<S> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let n = array1.len();
    let mut numerator = Dual::<S>::from(S::zero());
    let mut denominator = Dual::<S>::from(S::one());
    for i in 0..n {
        let coeff = <S as From<f64>>::from(rng.gen()); 
        let coeff = Dual::<S>::from(coeff); // generate a random coefficient
        numerator = numerator + coeff * array1[[i]].clone();
        denominator = denominator * (Dual::from(S::one()) + array1[[i]].clone());
    }
    
    EArray::<S,ndarray::Ix0>(ndarray::arr0(numerator / denominator))
}

use ndarray::{Array2};

fn compute_jacobian<S: epsilon_ad::Scalar>(seed: u64, array1: ArrayView1<S>) -> Array2<S> {
    let n = array1.len();
    let mut jacobian = Array2::<S>::zeros((1, n));
    for i in 0..n {
        let h = <S as From<f64>>::from(1e-8); // step size for numerical differentiation
        let mut x = array1.to_owned();
        x[i] = x[i] + h;
        let f_plus_h = create_rational_function(seed, (&x).into()); // evaluate the function at x + h
        x[i] = x[i] - <S as From<f64>>::from(2.0) * h;
        let f_minus_h = create_rational_function(seed, (&x).into()); // evaluate the function at x - h
        jacobian[[0, i]] = (f_plus_h - f_minus_h) / (<S as From<f64>>::from(2.0) * h); // compute the partial derivative using numerical differentiation
    }
    jacobian
}

use ndarray::{Array1};
use rand::{Rng, SeedableRng};

#[test]
fn main() {
    let seed = 0; 
    let n = 5; // length of input array
    let num_iter = 1000; // number of times to run the functions
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut results = Vec::<(f64, Array2<f64>)>::new(); // vector to store results
    
    for i in 0..num_iter {

        let input_array = Array1::from_shape_fn(n, |_| rng.gen_range(-0.5..0.5)); // generate random input array
        let f_val = create_rational_function(seed, (&input_array).into()); // evaluate the rational function
        let jacobian = compute_jacobian(seed, (&input_array).into()); // compute the Jacobian matrix
        // results.push((f_val, jacobian)); // store the results in a tuple and push onto the vector

        // let epsilon_input_array = input_array;
        let jacobian_eval = epsilon_ad::differentiation::jacobian::jacobian(|x| epsilon_create_rational_function(seed, x), &input_array);
        
        let tol = 1e-14;
        assert!(abs(jacobian_eval.value[ndarray::Dim(())] - f_val) < tol);

        let tol = 1e-5;
        let jacobian_diff = (&jacobian-jacobian_eval.jacobian).norm().square() / jacobian.len() as f64;
        assert!(jacobian_diff < tol, "Avg. distance of jacobians: {}", jacobian_diff);

    }
    
    // print out the results
//     for (i, (f_val, jacobian)) in results.iter().enumerate() {
//         println!("Result {}: f_val = {}\nJacobian:\n{}", i+1, f_val, jacobian);
//     }
}

