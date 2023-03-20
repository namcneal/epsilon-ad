// use bitvec::prelude::*;
use std::collections::HashMap;

pub type EpsilonID = usize;
pub type EpsilonDirection = usize;
pub type EpsilonProductID = EpsilonID;

#[derive(Debug, Clone)]
pub struct EpsilonProductData{
        id : EpsilonProductID,
    roster : HashMap<EpsilonID, EpsilonDirection>
}


#[derive(Debug, Clone)]
pub struct EpsilonProduct(pub Option<EpsilonProductData>);

impl EpsilonProduct{
    pub fn from_data(data:EpsilonProductData) -> EpsilonProduct{
        EpsilonProduct(Some(data))
    }
}

pub type Epsilon = EpsilonProduct;

impl Epsilon{
    pub fn new(id : usize, direction : usize) -> Epsilon{
        let data = EpsilonProductData{    id : id, 
                                                          roster : HashMap::from([(id, direction)])};
                                                          
        EpsilonProduct::from_data(data)
    }
}

use std::ops::Mul;
impl Mul<EpsilonProductData> for EpsilonProductData{
    type Output = EpsilonProduct;

    fn mul(self, rhs:Self) -> Self::Output{
        let new_id : usize = self.id ^ rhs.id;

        match new_id{
            0 => EpsilonProduct(None),
            _ => EpsilonProduct(Some(EpsilonProductData{ id      : new_id,
                                                         roster  : self.roster.into_iter()
                                                                         .chain(rhs.roster)
                                                                         .collect()
                 }))
        }

    }
}

impl Mul for EpsilonProduct{
    type Output = Self;

    fn mul(self, rhs:Self) -> Self::Output{
        match (self.0, rhs.0){
            (Some(_self), Some(_rhs)) => _self * _rhs,
            _ => EpsilonProduct(None)
        }
        
    }
}
