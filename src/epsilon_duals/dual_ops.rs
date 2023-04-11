use std::ops::*;
use std::todo;
use crate::epsilon_duals::scalar::Scalar;
use crate::epsilon_duals::duals::*;

impl Mul for NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> EpsilonProduct{
    let result = self.0^rhs.0;
        if result == 0{
            EpsilonProduct(None)
        } else { 
            EpsilonProduct(Some(NonEmptyEpsilonProduct(result))) 
        }
    }
}

impl Mul for EpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self:Self, rhs:Self) -> Self{
        match(self.0,rhs.0){
            (Some(a), Some(b)) => a*b,
            (_,_) => EpsilonProduct(None)
        }
    }
}

impl<T: Scalar> Mul for Perturbation<T> {
    
    type Output = Perturbation<T>;

    fn mul(self, rhs: Self) -> Self::Output {

        let mut coeffs = PerturbationData::<T>::new();
        let mut products = PerturbationData::<NonEmptyEpsilonProduct>::new();

        for (i, a) in self.terms.iter().enumerate() {
            for (j, b) in rhs.terms.iter().enumerate(){
                match (*a * *b).0{
                    Some(epsilon_product) => {
                        products.push(epsilon_product);
                        coeffs.push(self.coeffs[i] * rhs.coeffs[j])
                    }
                    None => (),
                }
            }
        }

        Perturbation::<T>{coeffs: coeffs, terms:products}

    }
}


/* 
    PROBLEM: We do not have a way to remove epsilons from a product. 
             The individual epsilons are XOR'd together so we can tell if
             a product term contains a specific one. However, this loses information
             about the full contents, so we cannot recover what the product would have been
             if we had not placed the epsilon there. 

             So suppose we have e1e2 and just e2 in the same place. If we have already implicitly
             extracted e1, then these should be the same epsilon. But that could not be known. 

             How fix?


 */
impl<T: Scalar> Add for Perturbation<T>{
    type Output = Perturbation<T>;

    // https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
    fn add(self, rhs: Self) -> Self::Output {
        let mut added_coeffs = PerturbationData::<T>::new();
        let mut added_terms  = self.terms.clone();
        
        // Iterate through the RHS to find matches in the LHS,
        // We want to see if a term's combination of individual epsilons is the same
        for (i, product) in rhs.terms.iter().enumerate(){
            match self.terms.binary_search(product){

                // If this combination of epsilons is already present, 
                // add the rhs's value to the existing combination
                Ok(here_index) => added_coeffs[here_index] = added_coeffs[here_index] + rhs.coeffs[i],
                
                // Otherwise place the value and the product in the right place 
                // to retain sortedness
                Err(could_be_index) => {
                    added_coeffs.insert(could_be_index, rhs.coeffs[i]);
                    added_terms.insert(could_be_index, rhs.terms[i]);
                }
            }
        }

        Perturbation::<T>{coeffs : added_coeffs, terms : added_terms}
    }
}


