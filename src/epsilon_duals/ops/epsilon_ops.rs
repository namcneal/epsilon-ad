use num::Zero;

use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        println!("\n{:?}, {:?}", self, rhs);
        let new_product_repr = self.bits() | rhs.bits();

        let final_product : EpsilonProduct;
        let lhs_rhs_same_order_same_direction = new_product_repr != self.bits() ^ rhs.bits();
        match lhs_rhs_same_order_same_direction{
            true => final_product = EpsilonProduct(None),

            _    => {
                let new_product = NonEmptyEpsilonProduct(new_product_repr);
                match  new_product.contains_multiple_epsilons_of_same_order(){
                    true => final_product = EpsilonProduct(None),
                    _    => final_product = EpsilonProduct(Some(new_product))
                }
            }
        }

        println!("{:?}\n", final_product);
        return final_product

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