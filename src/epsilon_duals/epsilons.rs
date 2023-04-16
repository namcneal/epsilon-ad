use std::ops::{Mul,Deref,DerefMut};


type DerivativeInvocationID = u64;
type DerivativeDirection = u64;

#[derive(Debug)]
pub struct EpsilonID(pub DerivativeInvocationID, pub DerivativeDirection);
type ReducedEpsilonID = u128;



impl EpsilonID{
    pub fn reduce(self:&Self) -> ReducedEpsilonID{
        // Stack the two vertically into a single 128 bit 
        // The direction (1) gets the highs and the id gets the lows (1)
        self.0 as u128 + ((self.1 as u128) << 64)
    }
}

type AggregateID = u128;

#[derive(Debug, Copy,Clone)]
pub struct NonEmptyEpsilonProduct{
    pub epsilons_within : AggregateID,
    pub num_epsilons : u8
}

impl NonEmptyEpsilonProduct {
    pub fn singleton(invocation_id:u64, direction:u64) -> NonEmptyEpsilonProduct{
        NonEmptyEpsilonProduct{epsilons_within : EpsilonID(invocation_id, direction).reduce(), num_epsilons : 1}
    }

    pub fn id(&self) -> AggregateID{
        self.epsilons_within
    }

    pub fn decrement(&mut self, sure:bool){
        if sure{
            self.num_epsilons -= 1;
        } 
        else{
            panic!("Are you sure you want to decrement this product?")
        }
    }
}   

impl PartialEq for NonEmptyEpsilonProduct{
    fn eq(&self, other: &Self) -> bool {
        (self.epsilons_within ^ other.epsilons_within) > 0
    }
}

pub struct EpsilonProduct(pub (super) Option<NonEmptyEpsilonProduct>);

impl Deref for EpsilonProduct{
    type Target = Option<NonEmptyEpsilonProduct>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EpsilonProduct{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<NonEmptyEpsilonProduct> for EpsilonProduct{
    fn from(value: NonEmptyEpsilonProduct) -> Self {
        EpsilonProduct(Some(value))
    }
}

impl Mul for &NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self::Output{
        let new_agg_id : u128 = self.epsilons_within ^ rhs.epsilons_within;

        // Get the low bits as these pertain just to the derivative invocations
        match new_agg_id as u64{
            0 => EpsilonProduct(None),
            _ => EpsilonProduct::from(NonEmptyEpsilonProduct {epsilons_within: new_agg_id, num_epsilons:  self.num_epsilons+rhs.num_epsilons})
        }
    }
}