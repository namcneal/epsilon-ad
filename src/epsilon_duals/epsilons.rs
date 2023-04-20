use std::ops::{Mul,Deref,DerefMut};


type DerivativeDepth     = u16;
type DerivativeDirection = u16;

#[derive(Debug)]
pub struct EpsilonID(pub DerivativeDepth, pub DerivativeDirection);
pub (crate) type ReducedEpsilonID = u32;

impl EpsilonID{
    pub fn reduce(self:&Self) -> ReducedEpsilonID{
        // Stack the two into a single 32 bit number
        self.0 as u32 | ((self.1 as u32) << 16)
    }
}

pub (crate) type AggregatedEpsilons = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyEpsilonProduct(pub (crate) AggregatedEpsilons);
pub (crate) type Epsilon = NonEmptyEpsilonProduct;


impl NonEmptyEpsilonProduct {
    pub fn singleton(order:u16, direction:u16) -> NonEmptyEpsilonProduct{
        NonEmptyEpsilonProduct(EpsilonID(order, direction).reduce())
    }
}   


#[derive(Debug)]
pub struct EpsilonProduct(pub (crate) Option<NonEmptyEpsilonProduct>);

impl From<NonEmptyEpsilonProduct> for EpsilonProduct{
    fn from(value: NonEmptyEpsilonProduct) -> Self {
        EpsilonProduct(Some(value))
    }
}

