use std::ops::{Mul,Deref,DerefMut};


const MAX_DIMENSION        : usize = 16;
const MAX_DERIVATIVE_ORDER : usize = 8;
type EpsilonOrder        =  usize;
type RepresentationAsInt = u16;
struct EpsilonBasisElement{
    bit_representation : RepresentationAsInt
}

impl EpsilonBasisElement{
    fn new(dimension:usize, direction: usize, order: usize) -> Result<EpsilonBasisElement, ()> {
        // let order_not_supported_msg = "Sorry, derivatives of order greater than 8 are not currently supported.";
        // let order_received_msg = format!("Expected an order <= 16. Received {}", order);
        assert!(dimension < MAX_DIMENSION);
        assert!(direction < MAX_DIMENSION);
        assert!(order < MAX_DERIVATIVE_ORDER ); 

        let mut representation : u16 = 1;
        representation <<= direction;

        Ok(EpsilonBasisElement{bit_representation : representation})
    }
}

pub (crate) type EpsilonProductRepresentation = u128;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NonEmptyEpsilonProduct(pub (crate) EpsilonProductRepresentation);
pub (crate) type Epsilon = NonEmptyEpsilonProduct;

impl NonEmptyEpsilonProduct {
    pub (crate) fn id(&self) -> EpsilonProductRepresentation{
        self.0
    }

    pub (crate) fn bits(&self) -> EpsilonProductRepresentation{
        self.0
    }

    fn shift_kth_order_place_to_first(&self, k:usize) -> EpsilonProductRepresentation{
        &self.bits() >> (k - 1) *  MAX_DIMENSION
    }

    fn first_order_place_contains_mutiple_epsilons_of_same_order(bit_repr:EpsilonProductRepresentation) -> bool{
        let bits_in_first_place = bit_repr as RepresentationAsInt; // truncate to first 
        bits_in_first_place.count_ones() > 1
    }

    fn kth_order_place_contains_muiltiple_epsilons_of_same_order(&self, k:usize) -> bool{
        let shifted_to_first = self.shift_kth_order_place_to_first(k);
        Self::first_order_place_contains_mutiple_epsilons_of_same_order(shifted_to_first)
    }

    pub (crate) fn contains_multiple_epsilons_of_same_order(&self) -> bool{
        let mut multiples_detected = false;
        for k in 1..=MAX_DERIVATIVE_ORDER{
            multiples_detected = multiples_detected || self.kth_order_place_contains_muiltiple_epsilons_of_same_order(k);
        }

        multiples_detected
    }

    pub fn basis_element(dimension:usize, direction:usize, order:usize) -> NonEmptyEpsilonProduct{
        let epsilon = EpsilonBasisElement::new(dimension,direction,order);
        match epsilon{
            Err(error) =>  panic!("Could not create epsilon basis element"),
            
            Ok(element) => {
                let mut full_representation_with_other_orders = element.bit_representation as u128;
                full_representation_with_other_orders <<= MAX_DIMENSION * (order - 1);
                
                NonEmptyEpsilonProduct(full_representation_with_other_orders)
            }
        }
    }
}   

impl Deref for NonEmptyEpsilonProduct{
    type Target = EpsilonProductRepresentation;

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

