use std::ops::{Deref, DerefMut};
use ndarray::{arr0, arr1,Array0, Array1, ArrayD, Array,ArrayView1,stack,Axis,Dim,Dimension, Ix1};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EArray<T: Scalar, D:Dimension>(pub Array<Dual<T>, D>);

pub type EReal<T>   = EArray<T,ndarray::Ix0>;
pub type Ef64       = EReal<f64>;
pub type EVector<T> = EArray<T, ndarray::Ix1>;
pub type EMatrix<T> = EArray<T, ndarray::Ix2>;
pub type EArrayD<T> = EArray<T,ndarray::IxDyn>;


impl<T: Scalar, D:Dimension> EArray<T,D>{
    pub fn values(&self) -> Array<T,D>{
        self.mapv(|el| el.value)
    }

    pub fn pow(&self, k:usize) -> Self{
        EArray::<T,D>((*self).map(|el| el.clone().pow(k)))
    }
}

impl<T: Scalar,D: ndarray::Dimension> Deref for EArray<T,D>{
    type Target = ndarray::Array<Dual<T>, D>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Scalar,D: ndarray::Dimension> DerefMut for EArray<T, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Scalar> From<Dual<T>> for EReal<T>{
	fn from(val:Dual<T>) -> EReal<T>{
		EArray::<T,ndarray::Ix0>(arr0(Dual::<T>::from(val)))
	}
}

impl<T: Scalar> From<T> for EReal<T>{
	fn from(val:T) -> EReal<T>{
        EReal::from(Dual::<T>::from(val))
    }
}

impl<T: Scalar> From<&[T]> for EVector<T>{
	fn from(s:&[T]) -> EVector<T>{
        <[T] as Lift<T,()>>::lift(s)
    }
}

impl<T: Scalar> From<&[Dual<T>]> for EVector<T>{
	fn from(s:&[Dual<T>]) -> EVector<T>{
        Self::from_dual_slice(s)
    }
}

impl<T: Scalar> From<&Array1<T>> for EVector<T>{
	fn from(s:&Array1<T>) -> EVector<T>{
        Self::from(s.as_slice().unwrap())
    }
}

impl<T: Scalar> From<&Array1<Dual<T>>> for EVector<T>{
	fn from(s:&Array1<Dual<T>>) -> EVector<T>{
        Self::from(s.as_slice().unwrap())
    }
}

impl<T: Scalar> EVector<T>{
    pub fn from_dual_slice(s:&[Dual<T>]) -> EVector<T>{
        EArray::<T,ndarray::Ix1>(arr1(s))
    }

    pub fn inner(u:&Self, v:&Self) -> EReal<T>{
        let inner : Dual<T> = 
            u.0.iter().zip(v.0.iter())
                .map(|(a,b)| a*b)
                .reduce(|acc,item| acc+item)
                .unwrap();

        EReal::<T>::from(inner)
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


impl<T: Scalar> EMatrix<T>{
    pub fn t(&self) -> Self{
        EArray::<T,ndarray::Ix2>(self.0.t().to_owned())
    }

    pub fn mat_mul(&self, rhs:&Self) -> Self{
        let lhs_shape = (*self).shape();
        let rhs_shape = (*rhs).shape();
        let mismatch_size_err = format!("Could not multiply matrices of shapes {:?} and {:?}", &lhs_shape, &rhs_shape);
        assert!(lhs_shape[1]==rhs_shape[0], "{}", &mismatch_size_err);

        let mut result = Array::from_elem([lhs_shape[0], rhs_shape[1]], Dual::<T>::zero());
        for i in 0..lhs_shape[0]{ for j in 0..rhs_shape[1]{

            for s in 0..lhs_shape[1]{
                let multiplied_elements = &(*self).slice(ndarray::s![i,s]) * &(*rhs).slice(ndarray::s![s,j]);
                let add_multiplied_elements_to_sum = &result.slice_mut(ndarray::s![i,j]) + &multiplied_elements;
                
                add_multiplied_elements_to_sum.assign_to(result.slice_mut(ndarray::s![i,j]));
            }
        }}

        return EArray::<T,ndarray::Ix2>(result);
    }
}