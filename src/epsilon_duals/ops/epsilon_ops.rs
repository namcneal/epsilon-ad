use num::Zero;

use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let new_product_repr = self.bits() | rhs.bits();
        let lhs_rhs_same_order_same_direction = new_product_repr != self.bits() ^ rhs.bits();

        let new_product = NonEmptyEpsilonProduct(new_product_repr);
        let product_will_cancel = new_product.contains_multiple_epsilons_of_same_order() || lhs_rhs_same_order_same_direction;

        match product_will_cancel{
            true => EpsilonProduct(None),
            _    => EpsilonProduct(Some(new_product))
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