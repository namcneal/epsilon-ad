use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let lhs_rhs_same_depth_and_direction =  *self == *rhs;

        match lhs_rhs_same_depth_and_direction{
            true => EpsilonProduct(None),
            _ => {
                let combined_orders = self[0] ^ rhs[0];
                let combined_directions = self[1] ^ rhs[1];
                EpsilonProduct::from(NonEmptyEpsilonProduct([combined_orders, combined_directions]))
            }
        }
    }
}

impl Mul for &EpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        match (self.0,rhs.0){
            (Some(lhs),Some(rhs)) => &lhs * &rhs,
            _ => EpsilonProduct(None)
        }
    }
}

impl Mul<&EpsilonProduct> for EpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:&Self) -> Self::Output{
        &self * rhs
    }
}