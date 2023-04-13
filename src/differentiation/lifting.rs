use crate::duals::{duals::*, perturbations::*};
use num::Float;
use num_traits::{Zero, One};
use std::ops::*;
use std::fmt::Debug;
use std::any::Any;


use crate::scalar::Scalar;

use duplicate::duplicate_item;

use ndarray::{Array, Array1, arr1};

type DerivativeID = u64;

pub trait Liftable<T> where T: Scalar{
    fn lift(&self, invocation_id:DerivativeID) -> Array1<Dual<T>>;
}

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Liftable<num_type> for num_type{
    fn lift(&self, _:DerivativeID) -> Array1<Dual<num_type>> {
        arr1(&[Dual::<num_type> { value: *self, 
                                     duals: Perturbation::<num_type>::empty_perturbation()}
              ]
            )
    }

}



impl<T: Scalar> Liftable<T> for Dual<T>{
    fn lift(&self, _:DerivativeID) -> Array1<Dual<T>>{
        arr1(&[self.clone()])
    }
}

impl<T:Scalar> Liftable<T> for Array1<T>{
    fn lift(&self, derivative_invocation_id:DerivativeID) -> Array1<Dual<T>>{
        let mut lifted : Array1<Dual<T>> = Array::from_elem((self.len(),), Dual::<T>::zero());

        for (direction, xi) in self.iter().enumerate(){
            let perturbation = Perturbation::<T>::singleton_product(derivative_invocation_id, direction as u64);
            lifted[direction] = Dual::<T>{value: *xi, duals: perturbation};
        }

        return lifted
    }
}






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