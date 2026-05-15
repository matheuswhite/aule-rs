use crate::math::from_f64::FromF64;
use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;

pub trait Scale: Sized {
    type Alpha: FromF64;
    fn scale(self, alpha: Self::Alpha) -> Self;
}

impl Scale for f32 {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> f32 {
        self * alpha
    }
}

impl Scale for f64 {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> f64 {
        self * alpha
    }
}

impl Scale for Complex<f32> {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> Complex<f32> {
        self * alpha
    }
}

impl Scale for Complex<f64> {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> Complex<f64> {
        self * alpha
    }
}

impl Scale for DMatrix<f32> {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> DMatrix<f32> {
        self * alpha
    }
}

impl Scale for DMatrix<f64> {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> DMatrix<f64> {
        self * alpha
    }
}

impl<const R: usize, const C: usize> Scale for SMatrix<f32, R, C> {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> SMatrix<f32, R, C> {
        self * alpha
    }
}

impl<const R: usize, const C: usize> Scale for SMatrix<f64, R, C> {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> SMatrix<f64, R, C> {
        self * alpha
    }
}
