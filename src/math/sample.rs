use crate::math::{float_point::FloatPoint, number::Number};
use core::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Neg, Sub},
};
use nalgebra::SMatrix;

pub trait Sample:
    Sized
    + Debug
    + Display
    + Clone
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Sum
{
    type Alpha: FloatPoint;

    fn zero() -> Self;
    fn one() -> Self;
    fn scale(&self, alpha: Self::Alpha) -> Self;
    fn lerp(start: &Self, end: &Self, alpha: Self::Alpha) -> Self;
    fn absolute(&self) -> Self;
    fn sinusoid(amplitude: &Self, omega_t: Self::Alpha, phase: &Self) -> Self;
    fn max_real() -> Self;
}

impl<T> Sample for T
where
    T: Number,
{
    type Alpha = T::Alpha;

    fn zero() -> Self {
        T::zero()
    }

    fn one() -> Self {
        T::one()
    }

    fn scale(&self, alpha: Self::Alpha) -> Self {
        Number::scale(*self, alpha)
    }

    fn lerp(start: &Self, end: &Self, alpha: Self::Alpha) -> Self {
        Number::lerp(*start, *end, alpha)
    }

    fn absolute(&self) -> Self {
        Number::absolute(*self)
    }

    fn sinusoid(amplitude: &Self, omega_t: Self::Alpha, phase: &Self) -> Self {
        Number::sinusoid(*amplitude, omega_t, *phase)
    }

    fn max_real() -> Self {
        Number::max_real()
    }
}

impl<T, const R: usize, const C: usize> Sample for SMatrix<T, R, C>
where
    T: Number + 'static,
{
    type Alpha = T::Alpha;

    fn zero() -> Self {
        SMatrix::from_fn(|_i, _j| T::zero())
    }

    fn one() -> Self {
        SMatrix::from_fn(|_i, _j| T::one())
    }

    fn scale(&self, alpha: Self::Alpha) -> Self {
        self.map(|x| x.scale(alpha))
    }

    fn lerp(start: &Self, end: &Self, alpha: Self::Alpha) -> Self {
        start.zip_map(end, |s, e| s + Sample::scale(&(e - s), alpha))
    }

    fn absolute(&self) -> Self {
        self.map(|x| x.absolute())
    }

    fn sinusoid(amplitude: &Self, omega_t: Self::Alpha, phase: &Self) -> Self {
        amplitude.zip_map(phase, |a, p| T::sinusoid(a, omega_t, p))
    }

    fn max_real() -> Self {
        SMatrix::from_fn(|_i, _j| T::max_real())
    }
}
