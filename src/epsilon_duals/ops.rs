use std::ops::*;
use crate::traits;
use crate::epsilon_duals::epsilon::*;
use crate::epsilon_duals::perturbation::*;
use crate::epsilon_duals::dual_numbers::*;

/*
    Operations related to Epsilons
 */

impl Mul<NonEmptyEpsilonProduct> for NonEmptyEpsilonProduct{
    type Output = EpsilonProduct;

    fn mul(self, rhs:Self) -> Self::Output{
        let new_id : usize = self.id ^ rhs.id;

        match new_id{
            0 => EpsilonProduct(None),
            _ => EpsilonProduct(Some(NonEmptyEpsilonProduct{ id      : new_id,
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

/*
    Operations related to Perturbations
 */

impl <T: traits::Scalar> AddAssign for Perturbation<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.coefficients.extend(rhs.coefficients);
        self.products.extend(rhs.products); 
    }
}

impl <T: traits::Scalar> Add for Perturbation<T>{
    type Output = Perturbation<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        return self;
    }
}

impl<T: traits::Scalar> Add<T> for Perturbation<T>{
    type Output = DualNumber<T>;

    fn add(self, rhs: T) -> Self::Output {
        DualNumber::<T>::Perturbed(rhs, self)
    }
}

impl<T: traits::Scalar> MulAssign<T> for Perturbation<T>{
    fn mul_assign(&mut self, rhs:T) -> (){
        for x in self.coefficients.iter_mut(){
            *x = (*x) * rhs.clone();
        }
    }
}

impl <T: traits::Scalar> Mul<T> for Perturbation<T>{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        return self;
    }
}

impl <T: traits::Scalar> Mul for Perturbation<T>{
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
                        new_products.push(EpsilonProduct::from_nonempty(data));
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



/*
    Operations related to Dual Numbers
 */

impl<T: traits::Scalar> Add<Perturbation<T>> for DualNumber<T>{
    type Output = DualNumber<T>;

    fn add(self, rhs: Perturbation<T>) -> Self::Output{
        match self{
            Self::Unperturbed(value) => DualNumber::Perturbed(value, rhs),
            Self::Perturbed(value, perturb) => DualNumber::Perturbed(value, perturb + rhs)
        }
    }

}

impl<T: traits::Scalar> Add for DualNumber<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Self::Unperturbed(value_a), Self::Unperturbed(value_b)) => {
                Self::Unperturbed(value_a+value_b)
            },

            (Self::Unperturbed(value_a), Self::Perturbed(value_b, perturb_b)) =>{
                Self::Perturbed(value_a+value_b, perturb_b)
            },

            (Self::Perturbed(value_a, perturb_a), Self::Unperturbed(value_b)) =>{
                Self::Perturbed(value_a+value_b, perturb_a)
            },

            (Self::Perturbed(value_a, perturb_a), Self::Perturbed(value_b, perturb_b)) =>{
                Self::Perturbed(value_a+value_b, perturb_a+perturb_b)
            }
        }
    }
}


impl<T: traits::Scalar> Mul for DualNumber<T>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match(self, rhs){
            (Self::Unperturbed(value_a), Self::Unperturbed(value_b)) =>{
                Self::Unperturbed(value_a*value_b)
            },

            (Self::Unperturbed(value_a), Self::Perturbed(value_b, perturb_b)) => {
                Self::Perturbed(value_a, perturb_b*value_b)
            },

            (Self::Perturbed(value_a, perturb_a), Self::Unperturbed(value_b)) => {
                Self::Perturbed(value_a*value_b, perturb_a*value_b)
            },

            (Self::Perturbed(value_a, perturb_a), Self::Perturbed(value_b, perturb_b)) => {
                Self::Perturbed(value_a*value_b, perturb_a.clone()*value_b + perturb_b.clone()*value_a + perturb_a*perturb_b) 
            }
        }
    }
}

