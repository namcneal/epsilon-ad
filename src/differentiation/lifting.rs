use crate::prelude::*;
use crate::epsilon_duals::perturbations::Perturbation;

use num::Float;
use num_traits::{Zero, One};
use std::ops::*;
use std::fmt::Debug;
use std::any::Any;


use crate::scalar::Scalar;

use duplicate::duplicate_item;

use ndarray::{Array, arr0, arr1,Array0,Array1,ArrayD,Dim};

pub (crate) type DerivativeID = u64;

pub trait Lift<T,D> 
where T: Scalar, D: ndarray::Dimension{
    fn lift(&self) -> EArray<T,D>;
}

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Lift<num_type,ndarray::Ix0> for num_type{
    fn lift(&self) -> EReal<num_type> {
        let dual = Dual::<num_type> { value: *self, 
                                      duals: Perturbation::<num_type>::empty_perturbation()};
        EArray::<num_type,ndarray::Ix0>(arr0(dual))
    }
}


impl<T:Scalar> Lift<T,ndarray::Ix1> for Vec<T>{
    fn lift(&self) -> EVector<T>{
        let lifted : Vec<Dual<T>> = self.iter().map(|el| Dual::<T>::from(*el)).collect();
        EArray::<T,ndarray::Ix1>(arr1(&lifted))
    }

}

impl<T:Scalar, D:ndarray::Dimension> Lift<T,D> for Array<T,D>{
    fn lift(&self) -> EArray<T,D>{
        EArray::<T,D>((*self).mapv(|el| Dual::<T>::from(el)))
    }
}



pub trait Unlift<T:Scalar>{
    type Target;
    fn unlift(&self) -> Self::Target;
}

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Unlift<num_type> for EReal<num_type>{
    type Target = num_type;
    fn unlift(&self) -> num_type {
        self[Dim(())].value
    }
}

impl<T:Scalar> Unlift<T> for EVector<T>{
    type Target = Array1<T>;
    fn unlift(&self) -> Self::Target {
        self.map(|el| el.value)
    }
}

impl<T:Scalar> Unlift<T> for EArrayD<T>{
    type Target = ArrayD<T>;
    fn unlift(&self) -> Self::Target {
        self.map(|el| el.value)
    }
}