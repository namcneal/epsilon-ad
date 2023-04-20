use std::ops::{Mul,Deref,DerefMut};


// type DerivativeDepth     = u16;
// type DerivativeDirection = u16;

// #[derive(Debug)]
// pub struct EpsilonID(pub DerivativeDepth, pub DerivativeDirection);
// pub (crate) type ReducedEpsilonID = u32;

// impl EpsilonID{
//     pub fn reduce(self:&Self) -> ReducedEpsilonID{
//         // Stack the two into a single 32 bit number
//         self.0 as u32 | ((self.1 as u32) << 16)
//     }
// }

// pub (crate) type AggregatedEpsilons = u32;

pub (crate) type EpsilonFieldType  = u8;
pub (crate) type EpsilonStoredType = u16;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyEpsilonProduct(pub (crate) EpsilonStoredType);
pub (crate) type Epsilon = NonEmptyEpsilonProduct;

impl NonEmptyEpsilonProduct {
    pub fn singleton(order:EpsilonFieldType, direction:EpsilonFieldType) -> NonEmptyEpsilonProduct{
        assert!(order > 0); assert!(direction > 0);

        let unique_pairing = Self::szuduki_pairing(order, direction);
        // println!("Ord.: {}  Dir.: {}   Pair: {}", order, direction, unique_pairing);
        NonEmptyEpsilonProduct(unique_pairing)
    }

    // https://codepen.io/sachmata/post/elegant-pairing
    fn szuduki_pairing(order:EpsilonFieldType, direction:EpsilonFieldType) -> EpsilonStoredType{
        let x = order     as EpsilonStoredType;
        let y = direction as EpsilonStoredType;
        if x >=y {
            x*x + x + y
        } else{
            y*y + x
        }
    }
}   

impl std::fmt::Debug for NonEmptyEpsilonProduct{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ε({})", self.0)
    }
}

impl Deref for NonEmptyEpsilonProduct{
    type Target = EpsilonStoredType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonEmptyEpsilonProduct{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}





#[derive(Debug)]
pub struct EpsilonProduct(pub (crate) Option<NonEmptyEpsilonProduct>);

impl From<NonEmptyEpsilonProduct> for EpsilonProduct{
    fn from(value: NonEmptyEpsilonProduct) -> Self {
        EpsilonProduct(Some(value))
    }
}

