use std::ops::{Deref, DerefMut};
use ndarray::{arr0, arr1,Array0, Array1, ArrayD, Array,ArrayView1,stack,Axis,Dim,Dimension, Ix1};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EArray<T: Scalar, D:Dimension>(pub (crate) Array<Dual<T>, D>);

impl<T: Scalar, D:Dimension> EArray<T,D>{
    pub fn pow(&self, k:usize) -> Self{
        EArray::<T,D>((*self).map(|el| el.clone().pow(k)))
    }
}


pub type EArrayD<T> = EArray<T,ndarray::IxDyn>;

impl<T: Scalar,D:Dimension> Deref for EArray<T,D>{
    type Target = Array<Dual<T>, D>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Scalar,D: Dimension> DerefMut for EArray<T, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


pub type EReal<T>   = EArray<T, ndarray::Ix0>;
pub type EVector<T> = EArray<T, ndarray::Ix1>;
pub type Ef64 = EReal<f64>;


impl<T: Scalar> From<T> for EReal<T>{
	fn from(val:T) -> EReal<T>{
		EArray::<T,ndarray::Ix0>(arr0(Dual::<T>::from(val)))
	}
}

impl<T: Scalar> From<Dual<T>> for EReal<T>{
	fn from(val:Dual<T>) -> EReal<T>{
		EArray::<T,ndarray::Ix0>(arr0(val))
	}
}

impl<T: Scalar> EVector<T>{
	pub fn from_vec(val:Vec<Dual<T>>) -> EVector<T>{
        EArray::<T,ndarray::Ix1>(arr1(&val))
    }

    pub fn from_arr(val:Array1<Dual<T>>) -> EVector<T>{
        EArray::<T,ndarray::Ix1>(val)
    }

    pub fn inner(u:&Self, v:&Self) -> EReal<T>{
        let inner : Array0<Dual<T>> = arr0(
            u.0.iter().zip(v.0.iter())
                .map(|(a,b)| a*b)
                .reduce(|acc,item| acc+item)
                .unwrap()
        );

        EArray::<T, ndarray::Ix0>(inner)
    }

    pub fn normalize(&mut self){
        let norm = Self::inner(&*self, &*self).0[Dim(())].sqrt();
        self.0.mapv_inplace(|u_i| u_i / norm.clone());
    }

    pub fn projection(of:&Self, onto:&Self) -> EVector<T>{
        let scale = &Self::inner(onto,of).0[Dim(())] / &Self::inner(onto,onto).0[Dim(())];
        // println!("{:?}", Self::inner(onto,onto));
        
        let mut projection = (*onto).clone();
        projection.0.mapv_inplace(|vi| scale.clone() * vi);

        return projection;
    }
}
