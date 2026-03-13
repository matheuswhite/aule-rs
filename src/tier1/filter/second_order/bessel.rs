use crate::{
    block::Block,
    prelude::{Biquad, Filter},
};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct Bessel<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    cutoff_freq: f64,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> Bessel<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    fn base_parameters(cutoff_freq: f64, dt: Duration) -> (f64, f64, f64) {
        let ts = dt.as_secs_f64();
        #[cfg(feature = "std")]
        let (k, d) = {
            let k = (core::f64::consts::PI * cutoff_freq * ts).tan();
            let d = 3f64.sqrt();

            (k, d)
        };
        #[cfg(not(feature = "std"))]
        let (k, d) = {
            let k = libm::tan(core::f64::consts::PI * cutoff_freq * ts);
            let d = libm::sqrt(3.0);

            (k, d)
        };
        let a0 = 1.0 + d * k + k * k;

        (k, d, a0)
    }

    pub fn low_pass(cutoff_freq: f64, dt: Duration) -> Self {
        let (k, d, a0) = Self::base_parameters(cutoff_freq, dt);

        let b0 = k * k / a0;
        let b1 = 2.0 * b0;
        let b2 = b0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (1.0 - d * k + k * k) / a0;

        Self {
            cutoff_freq,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn high_pass(cutoff_freq: f64, dt: Duration) -> Self {
        let (k, d, a0) = Self::base_parameters(cutoff_freq, dt);

        let b0 = 1.0 / a0;
        let b1 = -2.0 * b0;
        let b2 = b0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (k * k - d * k + 1.0) / a0;

        Self {
            cutoff_freq,
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

    pub fn bandwidth(&self) -> f64 {
        #[cfg(feature = "std")]
        {
            self.cutoff_freq / 3.0f64.sqrt()
        }
        #[cfg(not(feature = "std"))]
        {
            self.cutoff_freq / libm::sqrt(3.0)
        }
    }

    pub fn biquad_coefficients(&self) -> (f64, f64, f64, f64, f64) {
        self.biquad.coefficients()
    }
}

impl<T> Block for Bessel<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Input = T;
    type Output = T;

    fn output(
        &mut self,
        input: crate::prelude::Signal<Self::Input>,
    ) -> crate::prelude::Signal<Self::Output> {
        self.biquad.output(input)
    }

    fn reset(&mut self) {
        self.biquad.reset();
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.biquad.last_output()
    }
}

impl<T> Filter for Bessel<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
