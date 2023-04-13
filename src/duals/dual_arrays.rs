use crate::duals::duals::*;
use crate::scalar::*;
use ndarray::{Array1, ArrayD,ArrayView1,stack,Axis,Dim};

pub type DArray<T> = ArrayD<Dual<T>>;

#[derive(Debug, Clone)]
pub struct DVector<T: Scalar>(pub (crate) Array1<Dual<T>>);


impl<T: Scalar> DVector<T>{

    fn inner(u:&Self, v:&Self) -> Dual<T>{
        u.0.iter().zip(v.0.iter())
         .map(|(a,b)| a*b)
         .reduce(|acc,item| acc+item)
         .unwrap()
    }

    fn normalize(&mut self){
        let norm = Self::inner(&*self, &*self).sqrt();
        self.0.mapv_inplace(|u_i| u_i / norm.clone());
    }

    fn projection(of:&Self, onto:&Self) -> DVector<T>{
        let scale = Self::inner(onto,of) / Self::inner(onto,onto);
        // println!("{:?}", Self::inner(onto,onto));
        
        let mut projection = (*onto).clone();
        projection.0.mapv_inplace(|vi| scale.clone() * vi);

        return projection;
    }

    fn standard_basis_vector(dimension:usize, ith:usize) -> DVector<T>{
        let mut ei = Array1::from_elem(Dim([dimension]), Dual::<T>::from(T::zero()));
        ei[ith] = Dual::<T>::from(T::one());
        
        return DVector(ei);
    }

    fn standard_basis(dimension:usize) -> Vec<DVector<T>>{
        (0..dimension).map(|i| Self::standard_basis_vector(dimension, i)).collect()
    }

    pub (crate) fn graham_schmidt_with_standard_basis(initial_vector:&DVector<T>) -> ArrayD<Dual<T>>{
        let dimension = initial_vector.0.len();
        let original_basis = Self::standard_basis(dimension);
		let mut v = original_basis;

		v[0] = initial_vector.clone(); 
		
		let mut u = Vec::<DVector<T>>::new();
		u.push(DVector::<T>::from(v[0].clone()));

		for k in 1..dimension{
			let mut uk = v[k].clone();

			for ui in u.iter(){
				uk.0 = uk.0 - Self::projection(&v[k],&ui).0;
			}
			u.push(uk);
		}

        for basis_vec in u.iter_mut(){
            basis_vec.normalize();
        }


        // let result : Vec<Array1<Perturbation<T>>> = u.iter()
        //     .map(|dvec| dvec.0.mapv(|vi| vi.duals))
        //     .collect();

        let u_views : Vec<ArrayView1<Dual<T>>> = u.iter()
            .map(|v| v.0.view())
            .collect();

        stack(Axis(1), u_views.as_slice()).unwrap().into_dyn()

	}

}




