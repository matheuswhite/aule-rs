use crate::math::from_f64::FromF64;
use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;

pub trait Lerp: Sized {
    type Alpha: FromF64;
    fn lerp(start: &Self, end: &Self, alpha: Self::Alpha) -> Self;
}

impl Lerp for f32 {
    type Alpha = f32;
    fn lerp(start: &f32, end: &f32, alpha: f32) -> f32 {
        start + (end - start) * alpha
    }
}

impl Lerp for f64 {
    type Alpha = f64;
    fn lerp(start: &f64, end: &f64, alpha: f64) -> f64 {
        start + (end - start) * alpha
    }
}

impl Lerp for Complex<f32> {
    type Alpha = f32;
    fn lerp(start: &Complex<f32>, end: &Complex<f32>, alpha: f32) -> Complex<f32> {
        start + (end - start) * alpha
    }
}

impl Lerp for Complex<f64> {
    type Alpha = f64;
    fn lerp(start: &Complex<f64>, end: &Complex<f64>, alpha: f64) -> Complex<f64> {
        start + (end - start) * alpha
    }
}

impl Lerp for DMatrix<f32> {
    type Alpha = f32;
    fn lerp(start: &DMatrix<f32>, end: &DMatrix<f32>, alpha: f32) -> DMatrix<f32> {
        let r = start.nrows();
        let c = start.ncols();
        DMatrix::from_fn(r, c, |i, j| {
            start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha
        })
    }
}

impl Lerp for DMatrix<f64> {
    type Alpha = f64;
    fn lerp(start: &DMatrix<f64>, end: &DMatrix<f64>, alpha: f64) -> DMatrix<f64> {
        let r = start.nrows();
        let c = start.ncols();
        DMatrix::from_fn(r, c, |i, j| {
            start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha
        })
    }
}

impl<const R: usize, const C: usize> Lerp for SMatrix<f32, R, C> {
    type Alpha = f32;
    fn lerp(start: &Self, end: &Self, alpha: f32) -> Self {
        SMatrix::from_fn(|i, j| start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha)
    }
}

impl<const R: usize, const C: usize> Lerp for SMatrix<f64, R, C> {
    type Alpha = f64;
    fn lerp(start: &Self, end: &Self, alpha: f64) -> Self {
        SMatrix::from_fn(|i, j| start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha)
    }
}
