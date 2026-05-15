use crate::math::{from_f64::FromF64, from_usize::FromUsize, recip_of_count::RecipOfCount};
use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;

pub trait Scale: Sized {
    type Alpha: FromF64 + FromUsize + RecipOfCount + Copy;

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

impl Scale for DMatrix<Complex<f32>> {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> DMatrix<Complex<f32>> {
        self * Complex::new(alpha, 0.0)
    }
}

impl Scale for DMatrix<Complex<f64>> {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> DMatrix<Complex<f64>> {
        self * Complex::new(alpha, 0.0)
    }
}

impl<const R: usize, const C: usize> Scale for SMatrix<Complex<f32>, R, C> {
    type Alpha = f32;
    fn scale(self, alpha: f32) -> SMatrix<Complex<f32>, R, C> {
        self * Complex::new(alpha, 0.0)
    }
}

impl<const R: usize, const C: usize> Scale for SMatrix<Complex<f64>, R, C> {
    type Alpha = f64;
    fn scale(self, alpha: f64) -> SMatrix<Complex<f64>, R, C> {
        self * Complex::new(alpha, 0.0)
    }
}
