
use crate::traits;
use crate::epsilon_duals::epsilon::*;

#[derive(Debug, Clone)]
pub struct Perturbation<T: traits::Scalar>{
    pub (super) coefficients : Vec<T>,
    pub (super) products : Vec<EpsilonProduct>,
}

impl<T: traits::Scalar> Perturbation<T>{
    
    pub fn singleton(singleton_epsiolon_id:EpsilonProductID, direction:EpsilonDirection) -> Self{
        let epsilon = Epsilon::new(singleton_epsiolon_id, direction);
        Perturbation{coefficients : vec![T::one()], 
                     products     : vec![epsilon]
        }

    }
    
    pub (super) fn subperturbation_containing_id(perturbation:&Perturbation<T>, id:usize) -> Option<Perturbation<T>>{
        let mut products_with_id = Vec::<EpsilonProduct>::new();
        let mut coefficients_with_id = Vec::<T>::new();

        // Go through each product to check each one for the right id
        for (i, product) in perturbation.products.iter().enumerate(){

            match &product.0{
                // The empty product cannot the id, so an empty perturbation is returned
                None => return None,

                // Otherwise the product will actually have some data
                Some(product) => {
                    if NonEmptyEpsilonProduct::contains_id(&product, id){
                        products_with_id.push(EpsilonProduct::from_nonempty(product.clone()));
                        coefficients_with_id.push(perturbation.coefficients[i]);
                    }
                }
            }
        }

        Some(Perturbation::<T>{coefficients : coefficients_with_id, 
                          products     : products_with_id})
    }

}

