use crate::math::from_f64::FromF64;
use faer::Mat;
use faer::complex::Complex;

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

impl Lerp for Mat<f32> {
    type Alpha = f32;
    fn lerp(start: &Mat<f32>, end: &Mat<f32>, alpha: f32) -> Mat<f32> {
        let r = start.nrows();
        let c = start.ncols();
        Mat::from_fn(r, c, |i, j| {
            start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha
        })
    }
}

impl Lerp for Mat<f64> {
    type Alpha = f64;
    fn lerp(start: &Mat<f64>, end: &Mat<f64>, alpha: f64) -> Mat<f64> {
        let r = start.nrows();
        let c = start.ncols();
        Mat::from_fn(r, c, |i, j| {
            start[(i, j)] + (end[(i, j)] - start[(i, j)]) * alpha
        })
    }
}
