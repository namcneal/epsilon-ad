use crate::prelude::*;
use crate::Scalar;
use std::ops::*;

impl<T: Scalar, D:ndarray::Dimension> Add for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		EArray::<T,D>(&**self + &**rhs)
	}
}

impl<T: Scalar, D:ndarray::Dimension> Add for EArray<T,D>{
	type Output = EArray<T,D>;

	fn add(self, rhs: Self) -> Self::Output {
		&self + &rhs
	}
}

impl<T: Scalar, D: ndarray::Dimension> Add<T> for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn add(self, rhs: T) -> Self::Output {
		EArray::<T,D>(self.0.mapv(|el| el +rhs))
	}
}

impl<T: Scalar, D:ndarray::Dimension> Sub for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn sub(self, rhs: Self) -> Self::Output {
		EArray::<T,D>(&self.0 - &rhs.0)
	}
}


impl<T: Scalar, D:ndarray::Dimension> Sub for EArray<T,D>{
	type Output = EArray<T,D>;

	fn sub(self, rhs: Self) -> Self::Output {
		&self - &rhs
	}
}

impl<T: Scalar, D: ndarray::Dimension> Sub<T> for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn sub(self, rhs: T) -> Self::Output {
		EArray::<T,D>(self.0.mapv(|el| el +- rhs))
	}
}

impl<T: Scalar, D: ndarray::Dimension> Mul for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		EArray::<T,D>(&self.0 * &rhs.0)
	}
}

impl<T: Scalar, D: ndarray::Dimension> Mul for EArray<T,D>{
	type Output = EArray<T,D>;

	fn mul(self, rhs: Self) -> Self::Output {
		&self * &rhs
	}
}

impl<T: Scalar, D: ndarray::Dimension> Mul<T> for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn mul(self, rhs: T) -> Self::Output {
		EArray::<T,D>(self.0.mapv(|el| el * rhs))
	}
}

impl<T: Scalar, D: ndarray::Dimension> Div for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn div(self, rhs: Self) -> Self::Output {
		EArray::<T,D>(&self.0 / &rhs.0)
	}
}

impl<T: Scalar, D: ndarray::Dimension> Div for EArray<T,D>{
	type Output = EArray<T,D>;

	fn div(self, rhs: Self) -> Self::Output {
		&self / &rhs
	}
}

impl<T: Scalar, D: ndarray::Dimension> Div<T> for &EArray<T,D>{
	type Output = EArray<T,D>;

	fn div(self, rhs: T) -> Self::Output {
		EArray::<T,D>(self.0.mapv(|el| el / rhs))
	}
}