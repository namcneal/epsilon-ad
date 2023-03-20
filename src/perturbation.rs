#[path = "epsilon.rs"] mod epsilon; use epsilon::*;
#[path = "traits.rs" ] mod traits;  use crate::traits::Scalar;

use std::ops::*;


#[derive(Debug, Clone)]
pub struct Perturbation<T: Scalar>{
    coefficients : Vec<T>,
    products : Vec<EpsilonProduct>,
}

impl<T: Scalar> Perturbation<T>{
    
    pub fn singleton(singleton_epsiolon_id:EpsilonProductID, direction:EpsilonDirection) -> Self{
        let epsilon = Epsilon::new(singleton_epsiolon_id, direction);
        Perturbation{coefficients : vec![T::one()], 
                     products     : vec![epsilon]
        }

    }

}


impl <T: Scalar> AddAssign for Perturbation<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.coefficients.extend(rhs.coefficients);
        self.products.extend(rhs.products); 
    }
}


impl <T: Scalar> Add for Perturbation<T>{
    type Output = Perturbation<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        return self;
    }
}

impl<T: Scalar> MulAssign<T> for Perturbation<T>{
    fn mul_assign(&mut self, rhs:T) -> (){
        for x in self.coefficients.iter_mut(){
            *x = *x * rhs.clone();
        }
    }
}

impl <T: Scalar> Mul<T> for Perturbation<T>{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        return self;
    }
}

impl <T: Scalar> Mul for Perturbation<T>{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        let mut new_products = Vec::<EpsilonProduct>::new();
        let mut new_coefficients = Vec::<T>::new();

        if (self.products.len() == 0) | (rhs.products.len() == 0){
            return self
        }

        for i in 0..self.products.len(){
            for j in 0..self.products.len(){
                let product = self.products[i].clone() * rhs.products[j].clone();

                match product.0{
                    Some(data) => {
                        new_products.push(EpsilonProduct::from_data(data));
                        new_coefficients.push(self.coefficients[i] * rhs.coefficients[j]);
                    },
                    None => (),

                }
            }
        }

        self.coefficients = new_coefficients;
        self.products     = new_products;
        return self
    }
}
