use epsilon_ad::prelude::*;
use crate::epsilon_polynomials::monomials::*;
use std::ops::Mul;
use std::ops::Neg;

impl <T: epsilon_ad::Scalar, const D: usize> Neg for EMonomial<T,D>{
    type Output = EMonomial<T,D>;

    fn neg(mut self) -> Self::Output {
        self.coefficient = -self.coefficient;
        return self;
    }
}

impl<T: epsilon_ad::Scalar, const D: usize> Mul for &EMonomial<T,D>{
    type Output = EMonomial<T,D>;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_coefficient = self.coefficient * rhs.coefficient;
        let mut new_exponents : [u32; D]= [0; D];
        for i in 0..D{
            new_exponents[i] = self.exponents[i] + rhs.exponents[i];
        } 

        EMonomial::<T,D>{
            coefficient : new_coefficient,
            exponents:    new_exponents
        }
    }
}