use crate::{
    block::Block,
    prelude::{Biquad, Filter},
    signal::Signal,
};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct Chebyshev1<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    cutoff_freq: f64,
    ripple_db: f64,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> Chebyshev1<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    fn base_parameters(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> (f64, f64, f64) {
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

        #[cfg(feature = "std")]
        let wn = ((sinh_g * sinh_g + cosh_g * cosh_g) / 2.0).sqrt();
        #[cfg(not(feature = "std"))]
        let wn = libm::sqrt((sinh_g * sinh_g + cosh_g * cosh_g) / 2.0);

        let d = core::f64::consts::SQRT_2 * sinh_g / wn;

        #[cfg(feature = "std")]
        let k = (core::f64::consts::PI * cutoff_freq * ts).tan() * wn;
        #[cfg(not(feature = "std"))]
        let k = libm::tan(core::f64::consts::PI * cutoff_freq * ts) * wn;

        let a0 = k * k + d * k + 1.0;

        (d, k, a0)
    }

    pub fn low_pass(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> Self {
        let (d, k, a0) = Self::base_parameters(cutoff_freq, ripple_db, dt);

        let b0 = k * k / a0;
        let b1 = 2.0 * b0;
        let b2 = b0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (1.0 - d * k + k * k) / a0;

        Self {
            cutoff_freq,
            ripple_db,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn high_pass(cutoff_freq: f64, ripple_db: f64, dt: Duration) -> Self {
        let (d, k, a0) = Self::base_parameters(cutoff_freq, ripple_db, dt);

        let b0 = 1.0 / a0;
        let b1 = -2.0 * b0;
        let b2 = b0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (1.0 - d * k + k * k) / a0;

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

impl<T> Block for Chebyshev1<T>
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

impl<T> Filter for Chebyshev1<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
