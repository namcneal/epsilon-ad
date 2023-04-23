use num::Zero;

use crate::epsilon_duals::perturbations::*;
use crate::scalar::Scalar;
use std::iter::repeat;
use std::ops::*;
use std::thread::current;

#[derive(Clone, PartialEq)]
pub struct Dual<T: Scalar>{
    pub value : T,
    pub duals : Perturbation<T>
}

impl<T: Scalar> std::fmt::Debug for Dual<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut representation = String::new();
        representation.push_str(
            &format!("{:?}", self.duals)
        );
        representation.push_str(
            &format!("{:?}", self.value)
        );

        write!(f, "{}", &representation)

    }
}

impl<T: Scalar> From<T> for Dual<T>{
    fn from(value: T) -> Self {
        Dual::<T> { value: value, 
                    duals: Perturbation::<T>::empty_perturbation()}
        }
}

impl<T: Scalar> From<Perturbation<T>> for Dual<T>{
    fn from(value: Perturbation<T>) -> Self {
        Dual::<T> { value: T::zero(), 
                    duals: value}
        }
}

impl<T: Scalar> Dual<T>{
    pub fn zero() -> Self{
        Self::from(T::zero())
    }

    pub fn one() -> Self{
        Self::from(T::one())
    }

    // https://math.stackexchange.com/questions/3284393/what-is-the-taylor-series-of-a-square-root
    pub fn sqrt(&self) -> Self{

        let taylor_series_argument = self.duals.clone() /  self.value.powi(2);
        let mut current_argument = taylor_series_argument.clone();
        
        let mut n : u32 = 0;
        let mut catalan_n : u32 = 1;
        let mut taylor_series = Dual::<T>::one();
        while current_argument.products.len() > 0{
            let coefficient = catalan_n * (2 as u32).pow(2*n + 1) as u32;

            taylor_series = &taylor_series +  &(&current_argument * T::from(f64::powf(-1.0, n as f64) * coefficient as f64).unwrap()).into();

            n = n + 1;
            catalan_n = 2 * (2*n - 1) / (n + 1);
            current_argument = &current_argument * &taylor_series_argument;
            current_argument = current_argument.combine_like_monomials();

        }

        taylor_series * self.value
    }

    pub fn pow(&self, k:usize) -> Dual<T>{
        repeat(self)
            .take(k)
            .fold(Dual::<T>::one(), |acc,item| &acc*item)
    }
}

