use crate::epsilon_duals::epsilons::*;
use std::ops::Mul;

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let aggregated_epsilons = **self ^ **rhs;
        println!("LHS: {}   RHS: {}   Agg: {}", **self, **rhs, aggregated_epsilons);
        let lhs_rhs_contain_same_order_and_direction =  aggregated_epsilons == 0;

        match lhs_rhs_contain_same_order_and_direction{
            true => EpsilonProduct(None),
            _    => EpsilonProduct::from(NonEmptyEpsilonProduct(aggregated_epsilons))
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