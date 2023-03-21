// use bitvec::prelude::*;
use std::collections::HashMap;

pub type EpsilonID = usize;
pub type EpsilonDirection = usize;
pub type EpsilonProductID = EpsilonID;

#[derive(Debug, Clone)]
pub struct NonEmptyEpsilonProduct{
    pub (super) id : EpsilonProductID,
    pub (super) roster : HashMap<EpsilonID, EpsilonDirection>
}

#[derive(Debug, Clone)]
pub struct EpsilonProduct(pub Option<NonEmptyEpsilonProduct>);

impl NonEmptyEpsilonProduct{
    pub (super) fn contains_id(product:&NonEmptyEpsilonProduct, id:EpsilonID) -> bool{
        product.id ^ id == 0
    }
}

impl EpsilonProduct{
    pub (super) fn from_nonempty(product:NonEmptyEpsilonProduct) -> EpsilonProduct{
        EpsilonProduct(Some(product))
    }

}

pub type Epsilon = EpsilonProduct;

impl Epsilon{
    pub fn new(id : usize, direction : usize) -> Epsilon{
        let data = NonEmptyEpsilonProduct{    id : id, 
                                                          roster : HashMap::from([(id, direction)])};
                                                          
        EpsilonProduct::from_nonempty(data)
    }
}
