
use epsilon_ad::prelude::*;
use ndarray::{Array1, Array2, arr1};
use rand::{SeedableRng, Rng};
use rand::distributions::Uniform;
use epsilon_ad::Scalar;
use ndarray_linalg::Norm;
use num_traits::abs;

fn random_polynomial<T: Scalar, const D: usize>(seed: u64, x: &Array1<T>) -> T {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let degree = x.len();
    let coeffs: Vec<T> = (0..degree).map(|_| T::from(rng.sample(Uniform::new(-1.0, 1.0))).unwrap()).collect();
    let powers: Vec<Array1<usize>> = (0..degree).map(|i| {
        let mut power = arr1(&[0; D]);
        power[i] = 1;
        power
    }).collect();
    let mut result = T::from(0.0).unwrap();
    for (coeff, power) in coeffs.into_iter().zip(powers.into_iter()) {
        let dot_with = &power.mapv(|el| T::from(el as f64).unwrap());
        let term = coeff * x.iter().zip(dot_with.iter()).map(|(a,b)| *a**b).sum();
        result += term;
    }
    result
    
}

fn epsilon_random_polynomial<T: Scalar, const D: usize>(seed: u64, x: &EVector<T>) -> EReal<T> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let degree = x.len();
    let coeffs: Vec<T> = (0..degree).map(|_| T::from(rng.sample(Uniform::new(-1.0, 1.0))).unwrap()).collect();
    let powers: Vec<Array1<usize>> = (0..degree).map(|i| {
        let mut power = arr1(&[0; D]);
        power[i] = 1;
        power
    }).collect();
    let mut result = Dual::<T>::zero();
    for (coeff, power) in coeffs.into_iter().zip(powers.into_iter()) {
        let dot_with = &power.mapv(|el| T::from(el as f64).unwrap());
        let term = Dual::from(coeff) * x.iter().zip(dot_with.iter())
            .map(|(a,b)| a.clone()**b)
            .reduce(|acc,item| acc + item)
            .unwrap();

        result += term;
    }

    EReal::from(result)
}

use ndarray_linalg::{Determinant, Solve};

fn gpt_jacobian<T: Scalar, const D: usize>(seed: u64, x: &Array1<T>, f: &dyn Fn(u64, &Array1<T>) -> T) -> Array1<T> {
    let h = T::from(1e-6).unwrap(); // Step size for finite differences
    let n = x.len();
    let mut jacobian = Array1::from_elem(n, T::zero());
    let fx = f(seed, x);
    for i in 0..n {
        let mut xh = x.clone();
        xh[i] += h;
        let fxi = f(seed, &xh);
        let dfi = (fxi - fx) / h;
        jacobian.slice_mut(ndarray::s![i]).assign(&ndarray::arr0(dfi));
    }
    jacobian
}


fn gpt_hessian<T: Scalar, const D: usize>(seed: u64, x: &Array1<T>, f: &dyn Fn(u64, &Array1<T>) -> T) -> Array2<T> {
    let h = T::from(1e-6).unwrap(); // Step size for finite differences
    let n = x.len();
    let mut hessian = Array2::zeros((n, n));
    let fx = f(seed, x);
    for i in 0..n {
        let mut xh = x.clone();
        xh[i] += h;
        let fxi = f(seed, &xh);
        let dfi = (fxi - fx) / h;
        for j in i..n {
            let mut xhj = x.clone();
            xhj[j] += h;
            let fxj = f(seed, &xhj);
            let dfj = (fxj - fx) / h;
            let d2fij = (dfj - dfi) / h;
            hessian[(i, j)] = d2fij;
            hessian[(j, i)] = d2fij;
        }
    }
    hessian
}

#[test]
fn main() {
    const N: usize = 10;
    const D: usize = 3;
    let seed: u64 = 1;

    for i in 0..N {
        // Generate a random input vector
        let input: Array1<f64> = arr1(&rand::rngs::StdRng::seed_from_u64(seed + i as u64)
            .sample_iter(Uniform::new(-1.0, 1.0))
            .take(D)
            .collect::<Vec<f64>>());

        // Call the random_polynomial function on the input vector
        let output: f64 = random_polynomial::<f64,D>(seed + i as u64, &input);

        // Compute the Jacobian matrix of the random_polynomial function at the input vector
        let jacobian: Array1<f64> = gpt_jacobian::<f64,D>(seed + i as u64, &input,&random_polynomial::<f64,D>);

        // Compute the Hessian matrix of the random_polynomial function at the input vector
        let hessian: Array2<f64> = gpt_hessian::<f64,D>(seed + i as u64, &input, &random_polynomial::<f64,D>);

        let derivative_evaluation = DerivativeInvocation::<f64,2>::new(input.clone());
        let evaluated = derivative_evaluation.eval(|x| epsilon_random_polynomial::<f64,D>(seed+i as u64 , x));

        println!("{:?}", evaluated.output);
        let derivatives = evaluated.extract_all_derivatives();

        // Print the results
        println!("Iteration {}:", i);
        println!("Input: {:?}\n\n", input);
        println!("Output distance: {}", (output-derivatives.0).norm());
        println!("Jacobian distance: {:?}", (jacobian-&derivatives.1[0]).norm());
        println!("Hessian distance {:?}", (hessian-&derivatives.1[1]).norm());
        println!();
    }
}

