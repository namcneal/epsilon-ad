use std::ops::*;

use crate::epsilon_duals::epsilons::NonEmptyEpsilonProduct;
use crate::epsilon_duals::perturbations::*;
use crate::prelude::*;


/*
            Addition 
            --------
*/

impl<T: Scalar> Add for &Perturbation<T>{
    type Output = Perturbation<T>;

    // https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
    fn add(self, rhs: Self) -> Self::Output {
       let added_perturbation_data = Perturbation::combine_like_monomials(
            self.coefficients.iter().cloned().chain(rhs.coefficients.iter().cloned()).collect(), 
            self.products.iter().cloned().chain(rhs.products.iter().cloned()).collect()
        );
         
        
        Perturbation::<T>{coefficients : PerturbationData::from_vec(added_perturbation_data.0), 
                          products     : PerturbationData::from_vec(added_perturbation_data.1)}

    }
}

impl<T: Scalar> AddAssign for Perturbation<T>{
    // https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
    fn add_assign(&mut self, rhs: Self){
        // Iterate through the RHS to find matches in the LHS,
        // We want to see if a term's combination of individual epsilons is the same
        for i in 0..self.coefficients.len(){
            for (j, product) in rhs.products.iter().enumerate(){
                if self.products[i] == *product{
                    self.coefficients[i] += rhs.coefficients[j].clone();
                } else{
                    self.coefficients.push(rhs.coefficients[i].clone());
                    self.products.push(rhs.products[i].clone());
                }
            }
        }
    }
}

/*
            Subtraction 
            --------
*/

impl<T: Scalar> Sub for &Perturbation<T>{
    type Output = Perturbation<T>;

    // https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
    fn sub(self, rhs: Self) -> Self::Output {
        let mut added_coeffs = self.coefficients.clone();
        let mut added_terms  = self.products.clone();
        
        // Iterate through the RHS to find matches in the LHS,
        // We want to see if a term's combination of individual epsilons is the same
        for i in 0..added_terms.len(){
            for (j, product) in rhs.products.iter().enumerate(){
                if added_terms[i] == *product{
                    added_coeffs[i] = added_coeffs[i] - rhs.coefficients[j];
                } else{
                    added_coeffs.push(-rhs.coefficients[i]);
                    added_terms.push(rhs.products[i].clone());
                }
            }

        }

        Perturbation::<T>{coefficients : added_coeffs, products : added_terms}

    }
}


/*
            Multiplication 
            --------------
*/

impl<T: Scalar> Mul for &Perturbation<T> {
    
    type Output = Perturbation<T>;

    fn mul(self, rhs: Self) -> Self::Output {

        let mut coeffs = PerturbationData::<T>::new();
        let mut products = PerturbationData::<NonEmptyEpsilonProduct>::new();

        for (i, a) in self.products.iter().enumerate() {
            for (j, b) in rhs.products.iter().enumerate(){
                match &*(a * b){
                    Some(epsilon_product) => {
                        products.push(*epsilon_product);
                        coeffs.push(self.coefficients[i] * rhs.coefficients[j])
                    }
                    None => (),
                }
            }
        }

        Perturbation::<T>{coefficients: coeffs, products:products}

    }
}

impl<T: Scalar> Mul<T> for &mut Perturbation<T> {
    
    type Output = ();

    fn mul(self, rhs: T) -> Self::Output {
        for coeff in self.coefficients.iter_mut(){
            *coeff *= rhs
        }
    }
}

impl<T: Scalar> Mul<T> for &Perturbation<T> {
    
    type Output = Perturbation<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut new_coefficients = PerturbationData::<T>::new();
        for coeff in self.coefficients.iter(){
            new_coefficients.push(*coeff*rhs);
        }

        Perturbation::<T>{coefficients: new_coefficients, products: self.products.clone()}
    }
}

/*
            Division 
            --------
*/

impl <T: Scalar> Div<T> for &Perturbation<T>{
    type Output = Perturbation<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::mul(self, T::div(T::one(),rhs))
    }
}


impl<T: Scalar> Div<T> for Perturbation<T> {
    
    type Output = Perturbation<T>;

    fn div(self, rhs: T) -> Self::Output {
        <&Self>::div(&self, rhs)
    }
}



