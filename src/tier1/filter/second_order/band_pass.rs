use crate::{
    block::Block,
    prelude::{Biquad, Filter},
    signal::Signal,
};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct BandPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    center_freq: f64,
    q_factor: f64,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> BandPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(center_freq: f64, q_factor: f64, dt: Duration) -> Self {
        let ts = dt.as_secs_f64();

        #[cfg(feature = "std")]
        let k = (core::f64::consts::PI * center_freq * ts).tan();
        #[cfg(not(feature = "std"))]
        let k = libm::tan(core::f64::consts::PI * center_freq * ts);
        let a0 = 1.0 + k / q_factor + k * k;

        let b0 = k / q_factor / a0;
        let b1 = 0.0;
        let b2 = -b0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (1.0 - k / q_factor + k * k) / a0;

        Self {
            center_freq,
            q_factor,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn center_freq(&self) -> f64 {
        self.center_freq
    }

    pub fn q_factor(&self) -> f64 {
        self.q_factor
    }

    pub fn biquad_coefficients(&self) -> (f64, f64, f64, f64, f64) {
        self.biquad.coefficients()
    }
}

impl<T> Block for BandPass<T>
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

impl<T> Filter for BandPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
