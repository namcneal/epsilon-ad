use crate::epsilon_duals::duals::*;
use crate::epsilon_duals::perturbations::*;
use crate::scalar::Scalar;

use std::ops::*;
use num_traits::*;

/*
            Addition 
            --------
*/

impl<T: Scalar> Add for &Dual<T>{
    type Output = Dual<T>;

    fn add(self:Self, rhs: Self) -> Self::Output {
        Dual::<T>{value : self.value+rhs.value, duals: &self.duals + &rhs.duals}
    }
}

impl<T: Scalar> Add<T> for &Dual<T>{
    type Output = Dual<T>;

    fn add(self:Self, rhs: T) -> Self::Output {
        Dual::<T>{value : self.value+rhs, duals: self.duals.clone()}
    }
}

impl<T: Scalar> Add for Dual<T>{
    type Output = Dual<T>;

    fn add(self:Self, rhs: Self) -> Self::Output {
        <&Self>::add(&self, &rhs)

    }
}

impl<T: Scalar> Add<T> for Dual<T>{
    type Output = Dual<T>;

    fn add(self:Self, rhs: T) -> Self::Output {
        <&Self>::add(&self, rhs)

    }
}

impl<T: Scalar> AddAssign for Dual<T>{

    fn add_assign(self: &mut Self, rhs: Dual<T>){
        self.value += rhs.value;
        self.duals += rhs.duals;
    }
}

impl<T:Scalar> Zero for Dual<T>{
    fn is_zero(&self) -> bool {
        self.value.is_zero() && self.duals.products.len().is_zero()
    }

    fn zero() -> Self {
        Dual::<T>{value: T::zero(), duals: Perturbation::<T>::empty_perturbation()}
    }
}

/*
            Subtraction 
            -----------
*/

impl<T: Scalar> Sub for &Dual<T>{
    type Output = Dual<T>;

    fn sub(self:Self, rhs: Self) -> Self::Output {
        Dual::<T>{value : self.value-rhs.value, duals: &self.duals - &rhs.duals}

    }
}

impl<T: Scalar> Sub for Dual<T>{
    type Output = Dual<T>;

    fn sub(self:Self, rhs: Self) -> Self::Output {
        <&Self>::sub(&self, &rhs)
    }
}


/*
            Multiplication 
            --------------
*/

impl<T: Scalar> Mul for &Dual<T>{
    type Output = Dual<T>;

    fn mul(self:Self, rhs: Self) -> Self::Output {
        Dual::<T>{value: self.value*rhs.value, 
                  duals: &(&self.duals*rhs.value) + &(&rhs.duals*self.value)}
    }
}

impl<T: Scalar> Mul<T> for &Dual<T>{
    type Output = Dual<T>;

    fn mul(self:Self, rhs: T) -> Self::Output {
        Dual::<T>{value : self.value*rhs, duals: &self.duals * rhs}
    }
}

impl<T: Scalar> Mul for Dual<T>{
    type Output = Dual<T>;

    fn mul(self:Self, rhs: Self) -> Self::Output {
        <&Self>::mul(&self, &rhs)
    }
}

impl<T: Scalar> Mul<T> for Dual<T>{
    type Output = Dual<T>;

    fn mul(self:Self, rhs: T) -> Self::Output {
        Dual::<T>{value : self.value*rhs, duals: &self.duals * rhs}
    }
}


impl<T:Scalar> One for Dual<T>{
   fn one() -> Self {
        Dual::<T>{value: T::one(), 
                  duals: Perturbation::<T>::empty_perturbation()}
    }
}


/*
            Division 
            --------
*/

impl<T: Scalar> Div for &Dual<T>{
    type Output = Dual<T>;

    fn div(self:Self, rhs: Self) -> Self::Output {
        Dual::<T>{value: self.value/rhs.value, 
                  duals: &(&self.duals/rhs.value) + &(&rhs.duals*self.value/pow(rhs.value,2))}
    }
}

impl<T: Scalar> Div<T> for &Dual<T>{
    type Output = Dual<T>;

    fn div(self:Self, rhs: T) -> Self::Output {
        Dual::<T>{value: self.value/rhs, 
                  duals:&self.duals/rhs}
    }
}

impl<T: Scalar> Div for Dual<T>{
    type Output = Dual<T>;

    fn div(self:Self, rhs: Self) -> Self::Output {
        <&Self>::div(&self, &rhs)
    }
}

impl<T: Scalar> Div<T> for Dual<T>{
    type Output = Dual<T>;

    fn div(self:Self, rhs: T) -> Self::Output {
        Dual::<T>{value: self.value/rhs, 
                  duals:&self.duals/rhs}
    }
}

/*
            Remainder 
            ---------
*/
impl <T: Scalar> Rem for Dual<T>{
    type Output = Dual<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        &self - &(&self / &rhs)
    }
}


/*
            Num 
            ---
// */
impl<T: Scalar> Num for Dual<T>{
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let converted = T::from_str_radix(str, radix);
        match converted{
            Ok(result) =>{
                Ok(Dual::<T>{value: result,
                             duals: Perturbation::<T>::empty_perturbation()})
            },
            Err(_e) => Err(())
        }
        
    }
}




