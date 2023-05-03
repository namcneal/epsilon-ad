use crate::prelude::*;
use crate::Scalar;
use crate::epsilon_duals::epsilons::NonEmptyEpsilonProduct;
use crate::epsilon_duals::perturbations::*;

use std::ops::*;




/*
            Addition 
            --------
*/

impl<T: Scalar> Add for &Perturbation<T>{
    type Output = Perturbation<T>;

    // https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
    fn add(self, rhs: Self) -> Self::Output {
       Perturbation::combine_like_monomials_iter(
            self.coefficients.clone().into_iter().chain(rhs.coefficients.clone().into_iter()), 
            self.products.clone().into_iter().chain(rhs.products.clone().into_iter())
        )
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
        let mut negated_rhs = rhs.clone();
        for element in negated_rhs.coefficients.iter_mut(){
            *element = -*element;
        }

        self + &negated_rhs

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
                match (a * b).0{
                    Some(epsilon_product) => {
                        coeffs.push(self.coefficients[i] * rhs.coefficients[j]);
                        products.push(epsilon_product);
                    }
                    None => (),
                }
            }
        }

        Perturbation::<T>{coefficients: coeffs, products:products}.combine_like_monomials()

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

        let new_coefficients = smallvec::SmallVec::from(
            self.coefficients
                .iter()
                .map(|coeff:&T| *coeff * rhs)
                .collect::<Vec<T>>()
                .as_slice()
        );

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
        self *  (T::one() / rhs)
    }
}


impl<T: Scalar> Div<T> for Perturbation<T> {
    type Output = Perturbation<T>;

    fn div(self, rhs: T) -> Self::Output {
        &self / rhs
    }
}



