use crate::duals::{duals::*, perturbations::*};
use num::Float;
use num_traits::{Zero, One};
use std::ops::*;
use std::fmt::Debug;
use std::any::Any;


use crate::scalar::Scalar;

use duplicate::duplicate_item;

use ndarray::{Array, Array1, Array2};

type DerivativeID = u64;

pub trait Liftable<T> where T: Scalar{
    fn as_any(&self) -> &dyn Any;

    fn lift(&self) -> Dual<T>;
    // fn forgetful_unlift(&self) -> T;

    fn value(&self) -> T;
    fn duals(&self) -> Perturbation<T>;

    // fn neg(&self) -> Self;
}

#[duplicate_item(
    num_type;
    [f32];
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Liftable<num_type> for num_type{
    fn lift(&self) -> Dual<num_type> {
        Dual::<num_type> { value: *self, 
                           duals: Perturbation::<num_type>::empty_perturbation()}
    }

    fn value(&self) -> num_type {
        *self
    }

    fn duals(&self) -> Perturbation<num_type>{
        Perturbation::<num_type>::empty_perturbation()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



impl<T: Scalar> Liftable<T> for Dual<T>{
    fn lift(&self) -> Dual<T>{
        self.clone()
    }

    fn value(&self) -> T {
        self.value
    }

    fn duals(&self) -> Perturbation<T> {
        self.duals.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl<T: Scalar> Liftable<T> for Box<dyn Liftable<T>>{
    fn lift(&self) -> Dual<T> {
        (**self).lift()
    }

    fn value(&self) -> T {
        (**self).value()

    }

    fn duals(&self) -> Perturbation<T> {
        (**self).duals()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


// enum DerivativeDirection{
//     One(u64),
//     Array
// }

// pub trait Liftable{
//     fn lift_to_dual(&self, derivative_invocation_id:DerivativeID) -> Dual<Self>
//     where Self: Scalar;
//     // fn forgetful_unlift(&self) -> Self;
// }

// use num_bigfloat;
// use astro_float;

// #[duplicate_item(
//     num_type;
//     [f32];
//     [f64];
//     [num_bigfloat::BigFloat];
//     [astro_float::BigFloat];
// )]
// impl Liftable for num_type{
//     fn lift_to_dual(self:T, derivative_invocation_id:DerivativeID) -> Dual<T> {
//         Dual::<Self>::from(self)
//     }
// }

// impl Liftable for Array1<dyn Liftable>{
//     fn lift_to_dual(&self, derivative_invocation_id:DerivativeID) -> Dual<Self>
//         where Self : Scalar {
//             let mut lifted : Array1<Dual<Self>> = Array::from_elem((self.len(),), Dual::<Self>::zero());

//         for (direction, xi) in self.iter().enumerate(){
//             let perturbation = Perturbation::<Self>::singleton_product(derivative_invocation_id, direction as u64);
//             lifted[direction] = Dual::<Self>{value: *xi, duals: perturbation};
//         }

//         return lifted
        
//     }
// }






// pub trait LiftedArray<T: Scalar>{
//     fn lift_for_differentiation(self:Self,derivative_id:DerivativeID) -> Array1<Dual<T>>;
// }

// impl<'a, T: Scalar> LiftedArray<T> for Array1<T>{
    
//     fn lift_for_differentiation(self:Self,derivative_id:DerivativeID) -> Array1<Dual<T>> {
//         let mut lifted : Array1<Dual<T>> = Array::from_elem((self.len(),), Dual::<T>::zero());

//         for (direction, xi) in self.iter().enumerate(){
//             let perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
//             lifted[direction] = Dual::<T>{value: *xi, duals: perturbation};
//         }

//         return lifted
//     }
// }

// impl<'a, T:Scalar> LiftedArray<T> for Array1<Dual<T>>{
    
//     fn lift_for_differentiation(self:Self, derivative_id: DerivativeID) -> Array1<Dual<T>> {
//         let mut lifted : Array1<Dual<T>> = Array::from_elem((self.len(),), Dual::<T>::zero());

//         for (direction, xi) in self.iter().enumerate(){

//             let new_perturbation = Perturbation::<T>::singleton_product(derivative_id, direction as u64);
//             lifted[direction] = Dual::<T>{value: xi.value, duals: &new_perturbation + &(xi.duals)}
//         }

//         return lifted
//     }
// }

// macro_rules! lift_function{

//     (fn $name:ident<T:; $($rest:tt*)*) => {
//         lift_function!($rest*)
//     };

//     (fn $name:ident<T>($input:ident:&Array1<T>) -> Array1<T> $def:block) => {

//         fn $name<T: Scalar>($input: &Array1<Dual<T>>) -> Array1<Dual<T>> {$def}

//     };

//     (fn $name:ident<T:$trait_bounds:path>($input:ident:&Array1<T>) -> Array1<T> $def:block) => {

//         fn $name<T: Scalar + $trait_bounds>($input: &Array1<Dual<T>>) -> Array1<Dual<T>> {$def}

//     };
// }

// pub fn forgetful_unlift<T: Scalar>(lifted_array:&mut Array2<Dual<T>>) -> Array2<T>{
//     // let lifting_forgotten = ArrayD::<T>::zeros(lifted_array.shape());
//     lifted_array.map_mut(|dual| dual.value)
// }