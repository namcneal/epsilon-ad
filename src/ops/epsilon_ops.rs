use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let lhs_rhs_same_depth_and_direction =  *self == *rhs;

        match lhs_rhs_same_depth_and_direction{
            true => EpsilonProduct(None),
            _ => EpsilonProduct::from(NonEmptyEpsilonProduct(self.0 ^ rhs.0))
        }
    }
}