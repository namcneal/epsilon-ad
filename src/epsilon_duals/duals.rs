use std::cmp::Ord;

use smallvec::SmallVec;
use crate::epsilon_duals::scalar::Scalar;

#[derive(Debug)]
pub struct Epsilon(pub (super) [u128; 2]);

impl Epsilon{
    fn new(invocation_id : u64, direction : u64) -> Epsilon{
        let mut epsilon : [u128; 2] = [0,0];
        epsilon[0] += invocation_id as u128;
        epsilon[1] += direction as u128;

        Epsilon(epsilon)
    }

    fn invocation(self:&Self) -> u128{
        self.0[0]
    }

    fn direction(self:&Self) -> u128{
        self.0[1]
    }

    fn id(self:&Self) -> u128{
        self.szudzik_pairing()
    }

    fn szudzik_pairing(self:&Self) -> u128{
        let x = self.invocation();
        let y = self.direction();

        match x >= y{
            true => (x*x) + x + y,
               _ => y*2 + x
        }
    }
}

#[derive(Debug,Clone,Copy, PartialEq, Eq, PartialOrd, Ord)]
pub (super) struct NonEmptyEpsilonProduct(pub (super) u128);
type SingletonEpsilon = NonEmptyEpsilonProduct;

pub (super) struct EpsilonProduct(pub (super) Option<NonEmptyEpsilonProduct>);

impl From<Epsilon> for EpsilonProduct{
    fn from(eps:Epsilon) -> Self{
        EpsilonProduct(Some(NonEmptyEpsilonProduct(eps.id())))
    }
}

const SMALL_VEC_NUMBER_ELEMENTS : usize = 128;
pub (super) const SVNE : usize = SMALL_VEC_NUMBER_ELEMENTS;
pub (super) type PerturbationData<T> = SmallVec<[T; SVNE]>;

#[derive(Debug,Clone)]
pub (super) struct Perturbation<T>{
    pub (super) coeffs : PerturbationData<T>,
    pub (super) terms : PerturbationData<NonEmptyEpsilonProduct>
}

impl<T> Perturbation<T>{
    fn empty_perturbation()->Perturbation<T>{
        Perturbation::<T>{ coeffs: PerturbationData::<T>::new(), 
                           terms : PerturbationData::<NonEmptyEpsilonProduct>::new()}
    }
}

#[derive(Debug,Clone)]
pub struct Dual<T>{
    value : T,
    duals : Perturbation<T>
} 

impl<T: Scalar> From<T> for Dual<T>{
    fn from(value: T) -> Self {
        Dual::<T> { value: value, 
                    duals: Perturbation::<T>::empty_perturbation()}
    }
}