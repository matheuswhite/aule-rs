use crate::{
    block::Block,
    prelude::{Biquad, Filter},
    signal::Signal,
};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct Chebyshev2<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    cutoff_freq: f64,
    ripple_db: f64,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> Chebyshev2<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    fn base_parameters(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> (f64, f64, f64, f64) {
        let ts = dt.as_secs_f64();

        #[cfg(feature = "std")]
        let epsilon = (10f64.powf(ripple_db / 10.0) - 1.0).sqrt();
        #[cfg(not(feature = "std"))]
        let epsilon = libm::sqrt(libm::pow(10.0, ripple_db / 10.0) - 1.0);

        #[cfg(feature = "std")]
        let gamma = 0.5 * (1.0 / epsilon).asinh();
        #[cfg(not(feature = "std"))]
        let gamma = 0.5 * libm::asinh(1.0 / epsilon);

        #[cfg(feature = "std")]
        let (sinh_g, cosh_g) = (gamma.sinh(), gamma.cosh());
        #[cfg(not(feature = "std"))]
        let (sinh_g, cosh_g) = (libm::sinh(gamma), libm::cosh(gamma));

        // Type I pole locations (angle theta = pi/4)
        let sigma_1 = sinh_g / core::f64::consts::SQRT_2;
        let omega_1 = cosh_g / core::f64::consts::SQRT_2;
        let r1_sq = sigma_1 * sigma_1 + omega_1 * omega_1;

        // Type II: invert poles
        let sigma_2 = sigma_1 / r1_sq;
        let r2_sq = 1.0 / r1_sq;

        // Zeros at ±j/cos(π/4) = ±j√2 (normalized)
        let omega_z_sq = 2.0;

        #[cfg(feature = "std")]
        let k = (core::f64::consts::PI * cutoff_freq * ts).tan();
        #[cfg(not(feature = "std"))]
        let k = libm::tan(core::f64::consts::PI * cutoff_freq * ts);

        (sigma_2, r2_sq, omega_z_sq, k)
    }

    pub fn low_pass(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> Self {
        let (sigma_2, r2_sq, omega_z_sq, k) = Self::base_parameters(cutoff_freq, ripple_db, dt);

        // Denominator: H(s) denominator s² + 2σ₂s + r₂²
        let a0 = r2_sq * k * k + 2.0 * sigma_2 * k + 1.0;

        // Numerator with zeros: s² + ω_z²
        // Gain normalized for unity DC gain: multiply by r₂²/ω_z²
        let gain = r2_sq / omega_z_sq;
        let n0 = 1.0 + omega_z_sq * k * k;
        let n1 = 2.0 * (omega_z_sq * k * k - 1.0);

        let b0 = gain * n0 / a0;
        let b1 = gain * n1 / a0;
        let b2 = b0;
        let a1 = 2.0 * (r2_sq * k * k - 1.0) / a0;
        let a2 = (r2_sq * k * k - 2.0 * sigma_2 * k + 1.0) / a0;

        Self {
            cutoff_freq,
            ripple_db,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn high_pass(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> Self {
        let (sigma_2, r2_sq, omega_z_sq, k) = Self::base_parameters(cutoff_freq, ripple_db, dt);

        // HP denominator (LP→HP: s → 1/s)
        let a0 = r2_sq + 2.0 * sigma_2 * k + k * k;

        // HP numerator with zeros
        let gain = r2_sq / omega_z_sq;
        let n0 = omega_z_sq + k * k;
        let n1 = 2.0 * (k * k - omega_z_sq);

        let b0 = gain * n0 / a0;
        let b1 = gain * n1 / a0;
        let b2 = b0;
        let a1 = 2.0 * (k * k - r2_sq) / a0;
        let a2 = (r2_sq - 2.0 * sigma_2 * k + k * k) / a0;

        Self {
            cutoff_freq,
            ripple_db,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn cutoff_freq(&self) -> f64 {
        self.cutoff_freq
    }

    pub fn center_freq(&self) -> f64 {
        self.cutoff_freq
    }

    pub fn ripple_db(&self) -> f64 {
        self.ripple_db
    }

    pub fn biquad_coefficients(&self) -> (f64, f64, f64, f64, f64) {
        self.biquad.coefficients()
    }
}

impl<T> Block for Chebyshev2<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.biquad.output(input)
    }

    fn reset(&mut self) {
        self.biquad.reset();
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.biquad.last_output()
    }
}

impl<T> Filter for Chebyshev2<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
