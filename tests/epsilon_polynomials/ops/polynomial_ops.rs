use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::{polynomials::*, monomials::EMonomial};
use std::ops::{Add, Sub, Mul};


impl<T: epsilon_ad::Scalar, const D: usize> Add for EPolynomial<T,D>{
    type Output = EPolynomial<T,D>;

    fn add(self, rhs: Self) -> Self::Output {
        EPolynomial::<T,D>(self.0.into_iter().chain(rhs.0.into_iter()).collect())
    }
}

impl<T: epsilon_ad::Scalar, const D: usize> Sub for EPolynomial<T,D>{
    type Output = EPolynomial<T,D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let negated_rhs = rhs.clone().0.into_iter().map(|el| -el);
        EPolynomial::<T,D>(self.0.into_iter().chain(negated_rhs).collect())
    }
}

impl<T: epsilon_ad::Scalar, const D: usize> Mul for &EPolynomial<T,D>{
    type Output = EPolynomial<T,D>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut crossed_multiplied_monomials = Vec::<EMonomial<T,D>>::new();

        for monomial_a in self.0.iter(){
            for monomial_b in rhs.0.iter(){
                crossed_multiplied_monomials.push(monomial_a * monomial_b);
            }
        }

        EPolynomial::<T,D>(crossed_multiplied_monomials)
    }
}