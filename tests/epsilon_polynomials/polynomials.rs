use epsilon_polynomials::monomials;


pub struct Polynomial<T: Scalar, const D: usize, const N: usize>(pub (crate) [Monomial<T,D>; N]);




