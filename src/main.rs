#[path = "epsilon.rs"] mod epsilon;            use epsilon::*;
#[path = "perturbation.rs" ] mod perturbation; use crate::perturbation::Perturbation;
#[path = "traits.rs"       ] mod traits;       use crate::traits::Scalar;
use std::ops::*;
use num_traits;
use ndarray::Array1;


#[derive(Debug, Clone)]
enum DualNumber<T: traits::Scalar>{
    Simple(T, Perturbation<T>),
    Complex(Box<DualNumber<T>>, Perturbation<T>)
}

impl<T: traits::Scalar> Add<Perturbation<T>> for DualNumber<T>{
    type Output = DualNumber<T>;

    fn add(self, rhs: Perturbation<T>) -> Self::Output{
        match self{
            Self::Simple(val, per) => {
                DualNumber::<T>::Simple(val, per + rhs)
            },

            Self::Complex(dual,per) => {
                *dual + per + rhs
            }
        }
    }

}

impl<T: traits::Scalar> Mul<Perturbation<T>> for DualNumber<T>{
    type Output = DualNumber<T>;

    fn mul(self, rhs: Perturbation<T>) -> Self::Output {
        match self{
            Self::Simple(val, per) => {
                let scaled_perturbation : Perturbation<T> = rhs.clone()* val;
                let cross_multiplied    : Perturbation<T> = rhs * per;

                DualNumber::Simple(val, scaled_perturbation + cross_multiplied)
            }

           Self::Complex(dual, per) =>{
               *dual * rhs.clone() + rhs * per
           }
        }
    }
}

impl<T: traits::Scalar> Add for DualNumber<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs){
            (Self::Simple(va,pa), Self::Simple(vb, pb)) => Self::Simple(va+vb, pa+pb),
            
            (Self::Simple(va,pa), Self::Complex(dual, pb)) => Self::Simple(va,pa+pb) + *dual,

            (Self::Complex(dual,pa), Self::Simple(vb,pb)) => Self::Simple(vb,pa+pb) + *dual,

            (Self::Complex(dual_a, pa), Self::Complex(dual_b,pb )) => *dual_a + pa + *dual_b + pb,

        }
    }
}


impl<T: traits::Scalar> Mul for DualNumber<T>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match(self, rhs){
            (Self::Simple(va, pa), Self::Simple(vb, pb)) => Self::Simple(va.clone()*vb.clone(), pb*va+pa*vb),
            
            (Self::Simple(va, pa), Self::Complex(dual, pb)) =>
                match *dual{
                    Self::Simple(vc,pc) => Self::Simple(va.clone()*vc.clone(), pa*vc+pc*va),
                    Self::Complex(dual_c, pc) => {
                        "
                            TODO: Determine whether perturbation coefficients need to also be dual numbers.
                            This would mean a bit of a refactoring.
                        "
                    }
                }
        }
    }
}



trait DerivativeInput<T: Scalar> {
    fn lift_for_differentiation(value:Self, derivative_id:usize, direction:usize) -> DualNumber<T>;
}

impl<T: num_traits::Num + Scalar> DerivativeInput<T> for T{
    fn lift_for_differentiation(value:T, derivative_id:usize, direction:usize) -> DualNumber<T> {
        let perturbation = Perturbation::<T>::singleton(derivative_id, direction);
        DualNumber::<T>::Simple(value, perturbation)
    }
}

fn main() {
    let derivative_called : bool        = true; 
    let derivative_id     : *const bool = &derivative_called;
    let derivative_id     : usize       = derivative_id as usize;

    let direction : usize = 1;
    
    // let epsilon1 = Epsilon::new(1, 1);
    // let epsilon2 = Epsilon::new(1, 3);
    // let epsilon3 = Epsilon::new(2, 1);

    // println!("{:?}", epsilon1 * epsilon3);
    // println!("{:?}", epsilon1 * epsilon2 * epsilon3);

    // let null_product_canary = epsilon_id.
    // let invocation_id : usize = ;
    // println!("{}", arr);
}
