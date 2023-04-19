use std::ops::{Mul,Deref,DerefMut};


type DerivativeDepth     = u16;
type DerivativeDirection = u16;

#[derive(Debug)]
pub struct EpsilonID(pub DerivativeDepth, pub DerivativeDirection);
pub (crate) type ReducedEpsilonID = u32;
pub (crate) type EpsilonCount = u8;



impl EpsilonID{
    pub fn reduce(self:&Self) -> ReducedEpsilonID{
        // Stack the two vertically into a single 128 bit 
        // The direction (1) gets the highs and the id gets the lows (1)
        self.0 as u32 + ((self.1 as u32) << 16)
    }
}

pub (crate) type AggregatedEpsilons = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub (crate) struct NonEmptyEpsilonProduct(pub (crate) AggregatedEpsilons);
pub (crate) type Epsilon = NonEmptyEpsilonProduct;


impl NonEmptyEpsilonProduct {
    pub fn singleton(depth:u16, direction:u16) -> NonEmptyEpsilonProduct{
        NonEmptyEpsilonProduct(EpsilonID(depth, direction).reduce())
    }

    pub fn id(&self) -> AggregatedEpsilons{
        self.0
    }
}   


#[derive(Debug)]
pub struct EpsilonProduct(pub (crate) Option<NonEmptyEpsilonProduct>);

impl From<NonEmptyEpsilonProduct> for EpsilonProduct{
    fn from(value: NonEmptyEpsilonProduct) -> Self {
        EpsilonProduct(Some(value))
    }
}

