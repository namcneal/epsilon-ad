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

pub (crate) trait Lift<T,D> 
where T: Scalar{
    type Target;
    fn lift(&self) -> Self::Target;
}

#[duplicate_item(
    num_type;
    [f64];
    [num_bigfloat::BigFloat];
)]
impl Lift<num_type,ndarray::Ix0> for num_type{
    type Target = EReal<num_type>;
    
    fn lift(&self) -> Self::Target {
        let dual = Dual::<num_type> { value: *self, 
                                      duals: Perturbation::<num_type>::empty_perturbation()};
        EReal::<num_type>::from(dual)
    }
}


impl<T:Scalar> Lift<T,ndarray::Ix1> for Vec<T>{
    type Target = EVector<T>;

    fn lift(&self) -> Self::Target{
        let lifted : Vec<Dual<T>> = self.iter().map(|el| Dual::<T>::from(*el)).collect();
        EArray::<T,ndarray::Ix1>(arr1(&lifted))
    }

}

impl<T:Scalar, D:ndarray::Dimension> Lift<T,D> for Array<T,D>{
    type Target = EArray<T,D>;
    fn lift(&self) -> Self::Target{
        EArray::<T,D>((*self).mapv(|el| Dual::<T>::from(el)))
    }
}

impl<T:Scalar, D> Lift<T,D> for [T]{
    type Target = EVector<T>;
    fn lift(&self) -> Self::Target{
        arr1(self).lift()
    }
}



pub trait Unlift<T:Scalar>{
    type Target;
    fn unlift(&self) -> Self::Target;
}

impl<T:Scalar> Unlift<T> for EReal<T>{
    type Target = T;
    fn unlift(&self) -> T {
        (**self)[Dim(())].value
    }
}

impl<T:Scalar> Unlift<T> for EArrayD<T>{
    type Target = ArrayD<T>;
    fn unlift(&self) -> Self::Target {
        self.values()
    }
}