use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let new_aggregated_order_contents = self.aggregated_order_contents ^ rhs.aggregated_order_contents;
        let lhs_and_rhs_contain_same_order =  new_aggregated_order_contents == 0;

        match lhs_and_rhs_contain_same_order{
            true => EpsilonProduct(None),
            _    => {
                let new_product_bit_representation = self.product_bit_representation & rhs.product_bit_representation;
                let epsilon = NonEmptyEpsilonProduct{
                    aggregated_order_contents : new_aggregated_order_contents,
                    product_bit_representation : new_product_bit_representation
                };

                EpsilonProduct(Some(epsilon))
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