use crate::prelude::*;
use ndarray::{Array1, Dim,ArrayView1, stack, Axis};
pub struct StandardBasis<T: Scalar>(EVector<T>);


impl<T: Scalar> From<EVector<T>> for StandardBasis<T>{
    fn from(initial_vector: EVector<T>) -> Self {
        Self::from_vec(initial_vector)
    }
}
impl<T:Scalar> StandardBasis<T>{
    pub fn from_vec(initial_vector:EVector<T>) -> Self{
        StandardBasis(initial_vector)
    }

    fn standard_basis_vector(dimension:usize, ith:usize) -> EVector<T>{
        let mut ei = Array1::from_elem(Dim([dimension]), Dual::<T>::from(T::zero()));
        ei[ith] = Dual::<T>::from(T::one());
        
        return EArray::<T,ndarray::Ix1>(ei);
    }

    fn standard_basis_vectors(dimension:usize) -> Vec<EVector<T>>{
        (0..dimension).map(|i| Self::standard_basis_vector(dimension, i)).collect()
    }

    pub fn graham_schmidt_with_standard_basis(initial_vector:&EVector<T>) -> EArray<T,ndarray::Ix2>{
        let dimension = initial_vector.0.len();
        let original_basis = Self::standard_basis_vectors(dimension);
		let mut v = original_basis;

		v[0] = initial_vector.clone(); 
		
		let mut u = Vec::<EVector<T>>::new();
		u.push(EVector::<T>::from(v[0].clone()));

		for k in 1..dimension{
			let mut uk = v[k].clone();

			for ui in u.iter(){
				uk.0 = uk.0 - EVector::<T>::projection(&v[k],&ui).0;
			}
			u.push(uk);
		}

        let mut orthonormal_matrix: ndarray::Array2<Dual<T>> = ndarray::Array::from_elem(Dim([dimension,dimension]), Dual::<T>::zero());

        for (i,basis_vec )in u.iter_mut().enumerate(){
            basis_vec.normalize();
            basis_vec.assign_to(orthonormal_matrix.slice_mut(ndarray::s![i,..]));
        }

        return EArray::<T,ndarray::Ix2>(orthonormal_matrix)
	}

    pub fn matrix(&self) -> EArray<T,ndarray::Ix2>{
        Self::graham_schmidt_with_standard_basis(&self.0)
    }
}


