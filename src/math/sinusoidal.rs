use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;

/// Computes `amplitude * sin(omega_t + phase)` element-wise per type.
///
/// - Scalars: regular multiplication and `sin`.
/// - Complex: `phase` is complex (with imaginary perturbation), `sin` is the complex sine.
/// - Matrices: per-element sinusoid with per-element amplitude and phase, uniform `omega_t`.
pub trait Sinusoidal: Sized {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self;
}

impl Sinusoidal for f32 {
    fn sinusoid(amplitude: &f32, omega_t: f64, phase: &f32) -> f32 {
        *amplitude * libm::sinf(omega_t as f32 + *phase)
    }
}

impl Sinusoidal for f64 {
    fn sinusoid(amplitude: &f64, omega_t: f64, phase: &f64) -> f64 {
        *amplitude * libm::sin(omega_t + *phase)
    }
}

fn complex_sin_f32(z: Complex<f32>) -> Complex<f32> {
    Complex::new(
        libm::sinf(z.re) * libm::coshf(z.im),
        libm::cosf(z.re) * libm::sinhf(z.im),
    )
}

fn complex_sin_f64(z: Complex<f64>) -> Complex<f64> {
    Complex::new(
        libm::sin(z.re) * libm::cosh(z.im),
        libm::cos(z.re) * libm::sinh(z.im),
    )
}

impl Sinusoidal for Complex<f32> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let arg = *phase + Complex::new(omega_t as f32, 0.0);
        *amplitude * complex_sin_f32(arg)
    }
}

impl Sinusoidal for Complex<f64> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let arg = *phase + Complex::new(omega_t, 0.0);
        *amplitude * complex_sin_f64(arg)
    }
}

impl Sinusoidal for DMatrix<f32> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = omega_t as f32;
        let sin_arg = phase.map(|p| libm::sinf(omega_t + p));
        amplitude.component_mul(&sin_arg)
    }
}

impl Sinusoidal for DMatrix<f64> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let sin_arg = phase.map(|p| libm::sin(omega_t + p));
        amplitude.component_mul(&sin_arg)
    }
}

impl<const R: usize, const C: usize> Sinusoidal for SMatrix<f32, R, C> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = omega_t as f32;
        let sin_arg = phase.map(|p| libm::sinf(omega_t + p));
        amplitude.component_mul(&sin_arg)
    }
}

impl<const R: usize, const C: usize> Sinusoidal for SMatrix<f64, R, C> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let sin_arg = phase.map(|p| libm::sin(omega_t + p));
        amplitude.component_mul(&sin_arg)
    }
}

impl Sinusoidal for DMatrix<Complex<f32>> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = Complex::new(omega_t as f32, 0.0);
        let sin_arg = phase.map(|p| complex_sin_f32(p + omega_t));
        amplitude.component_mul(&sin_arg)
    }
}

impl Sinusoidal for DMatrix<Complex<f64>> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = Complex::new(omega_t, 0.0);
        let sin_arg = phase.map(|p| complex_sin_f64(p + omega_t));
        amplitude.component_mul(&sin_arg)
    }
}

impl<const R: usize, const C: usize> Sinusoidal for SMatrix<Complex<f32>, R, C> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = Complex::new(omega_t as f32, 0.0);
        let sin_arg = phase.map(|p| complex_sin_f32(p + omega_t));
        amplitude.component_mul(&sin_arg)
    }
}

impl<const R: usize, const C: usize> Sinusoidal for SMatrix<Complex<f64>, R, C> {
    fn sinusoid(amplitude: &Self, omega_t: f64, phase: &Self) -> Self {
        let omega_t = Complex::new(omega_t, 0.0);
        let sin_arg = phase.map(|p| complex_sin_f64(p + omega_t));
        amplitude.component_mul(&sin_arg)
    }
}
