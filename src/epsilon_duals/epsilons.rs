use std::ops::{Mul,Deref,DerefMut};


pub (crate) type EpsilonFieldType  = u8;
pub (crate) type EpsilonStoredType = u16;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyEpsilonProduct(pub (crate) EpsilonStoredType);
pub (crate) type Epsilon = NonEmptyEpsilonProduct;

impl NonEmptyEpsilonProduct {
    pub fn singleton(order:EpsilonFieldType, direction:EpsilonFieldType) -> NonEmptyEpsilonProduct{
        assert!(order > 0); assert!(direction > 0);

        // let unique_pairing = Self::szuduki_pairing(order, direction);
        // println!("Ord.: {}  Dir.: {}   Pair: {}", order, direction, unique_pairing);
        NonEmptyEpsilonProduct(Self::combine_fields_for_storage(order, direction))
    }

    pub fn combine_fields_for_storage(order:EpsilonFieldType, direction:EpsilonFieldType) -> EpsilonStoredType{
        // Stack the two into a single 32 bit number
        let shift = 8*std::mem::size_of::<EpsilonFieldType>();
        order as EpsilonStoredType | ((direction as EpsilonStoredType) << shift)
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

