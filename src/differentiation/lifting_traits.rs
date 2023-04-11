use crate::differentiation::lifting::*;
use crate::scalar::Scalar;
use std::fmt::Debug;
use num_traits::{Zero, One};
use std::ops::*;


impl<T: Scalar> Debug for Box<dyn Liftable<T>>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Scalar> Clone for Box<dyn Liftable<T>>{
    fn clone(&self) -> Self {
        self.clone()
    }
}

// impl<T: Scalar> PartialEq for Box<dyn Liftable<T>> {
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

// impl<T: Scalar> PartialOrd for Box<dyn Liftable<T>>{
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         (**self).value().partial_cmp(other)
//     }

//     fn ge(&self, other: &Self) -> bool {
//         self >= other
//     }
// }

// impl<T:Scalar> Neg for Box<&dyn Liftable<T>>{
//     type Output = Self;
//     fn neg(self) -> Self::Output {
//         *(*self).as_any()
//                .downcast_ref()
//                .expect("Could not downcast in Neg trait.")
//                .neg()
//     }
// }

// impl<T: Scalar> Add for Box<dyn Liftable<T>>{
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.add(rhs)
//     }
// }


// impl<T: Scalar> Sub for Box<dyn Liftable<T>>{
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         (*self).sub(rhs)
//     }
// }

// impl<T: Scalar> Mul for Box<dyn Liftable<T>>{
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         self*rhs
//     }
// }

// impl<T: Scalar> Div for Box<dyn Liftable<T>>{
//     type Output = Self;
//     fn div(self, rhs: Self) -> Self::Output {
//         self/rhs
//     }
// }

// impl <T: Scalar> AddAssign for Box<dyn Liftable<T>>{
//     fn add_assign(&mut self, rhs: Self) {
//         *self += rhs
//     }
// }

// impl <T: Scalar> MulAssign for Box<dyn Liftable<T>>{
//     fn mul_assign(&mut self, rhs: Self) {
//         *self *= rhs
//     }
// }

// impl<T: Scalar> Zero for Box<dyn Liftable<T>>{
//     fn zero() -> Self {
//         Self::zero()
//     }

//     fn is_zero(&self) -> bool {
//         self.is_one()
//     }
// }

// impl<T: Scalar> One for Box<dyn Liftable<T>>{
//     fn one() -> Self {
//         Box::new(Self::one())
//     }
// }



