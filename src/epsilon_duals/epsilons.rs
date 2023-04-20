use std::ops::{Mul,Deref,DerefMut};


const MAX_DIMENSION        : usize = 16;
const MAX_DERIVATIVE_ORDER : usize = 8;
type EpsilonOrder        =  usize;
struct EpsilonBasisElement{
    bit_representation : u16
}

impl EpsilonBasisElement{
    fn new(dimension:usize, direction: usize, order: usize) -> Result<EpsilonBasisElement, ()> {
        // let order_not_supported_msg = "Sorry, derivatives of order greater than 8 are not currently supported.";
        // let order_received_msg = format!("Expected an order <= 16. Received {}", order);
        assert!(dimension < MAX_DIMENSION);
        assert!(direction < MAX_DIMENSION);
        assert!(order > 1); 
        assert!(order < MAX_DERIVATIVE_ORDER ); 

        let mut representation : u16 = 1;
        representation <<= direction;

        Ok(EpsilonBasisElement{bit_representation : representation})
    }
}

pub (crate) type EpsilonProductRepresentation = u128;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NonEmptyEpsilonProduct{
    pub (crate) aggregated_order_contents  : u8,
    pub (crate) product_bit_representation : EpsilonProductRepresentation
}
pub (crate) type Epsilon = NonEmptyEpsilonProduct;

impl NonEmptyEpsilonProduct {
    pub (crate) fn id(&self) -> EpsilonProductRepresentation{
        self.product_bit_representation
    }

    pub fn basis_element(dimension:usize, direction:usize, order:usize) -> NonEmptyEpsilonProduct{
        let epsilon = EpsilonBasisElement::new(dimension,direction,order);
        match epsilon{
            Err(error) =>  panic!("Could not create epsilon basis element"),
            Ok(element) => {
                let mut full_representation_with_other_orders = element.bit_representation as u128;
                full_representation_with_other_orders <<= MAX_DIMENSION * (order - 1);
                NonEmptyEpsilonProduct{
                    aggregated_order_contents : order as u8, 
                    product_bit_representation : full_representation_with_other_orders
                                
                }
            }
        }
    }
}   


#[derive(Debug)]
pub struct EpsilonProduct(pub (crate) Option<NonEmptyEpsilonProduct>);

impl From<NonEmptyEpsilonProduct> for EpsilonProduct{
    fn from(value: NonEmptyEpsilonProduct) -> Self {
        EpsilonProduct(Some(value))
    }
}

