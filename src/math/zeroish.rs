use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;
use num_traits::Zero;

/// Produces a "zero-like" value with the same shape as `prototype`.
///
/// For scalar and statically-sized types, this is just `Zero::zero()` (ignores prototype).
/// For dynamically-sized types like `DMatrix<T>`, the prototype is used to determine shape.
pub trait Zeroish: Sized {
    fn zeroish(prototype: &Self) -> Self;
}

impl Zeroish for f32 {
    fn zeroish(_prototype: &Self) -> Self {
        f32::zero()
    }
}

impl Zeroish for f64 {
    fn zeroish(_prototype: &Self) -> Self {
        f64::zero()
    }
}

impl Zeroish for Complex<f32> {
    fn zeroish(_prototype: &Self) -> Self {
        Complex::<f32>::zero()
    }
}

impl Zeroish for Complex<f64> {
    fn zeroish(_prototype: &Self) -> Self {
        Complex::<f64>::zero()
    }
}

impl Zeroish for DMatrix<f32> {
    fn zeroish(prototype: &Self) -> Self {
        DMatrix::zeros(prototype.nrows(), prototype.ncols())
    }
}

impl Zeroish for DMatrix<f64> {
    fn zeroish(prototype: &Self) -> Self {
        DMatrix::zeros(prototype.nrows(), prototype.ncols())
    }
}

impl<const R: usize, const C: usize> Zeroish for SMatrix<f32, R, C> {
    fn zeroish(_prototype: &Self) -> Self {
        SMatrix::<f32, R, C>::zeros()
    }
}

impl<const R: usize, const C: usize> Zeroish for SMatrix<f64, R, C> {
    fn zeroish(_prototype: &Self) -> Self {
        SMatrix::<f64, R, C>::zeros()
    }
}

impl Zeroish for DMatrix<Complex<f32>> {
    fn zeroish(prototype: &Self) -> Self {
        DMatrix::zeros(prototype.nrows(), prototype.ncols())
    }
}

impl Zeroish for DMatrix<Complex<f64>> {
    fn zeroish(prototype: &Self) -> Self {
        DMatrix::zeros(prototype.nrows(), prototype.ncols())
    }
}

impl<const R: usize, const C: usize> Zeroish for SMatrix<Complex<f32>, R, C> {
    fn zeroish(_prototype: &Self) -> Self {
        SMatrix::<Complex<f32>, R, C>::zeros()
    }
}

impl<const R: usize, const C: usize> Zeroish for SMatrix<Complex<f64>, R, C> {
    fn zeroish(_prototype: &Self) -> Self {
        SMatrix::<Complex<f64>, R, C>::zeros()
    }
}
