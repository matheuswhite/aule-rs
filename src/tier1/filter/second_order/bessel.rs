use crate::{
    block::Block,
    math::{float_point::AsFloatPoint, float_point::FloatPoint, sample::Sample},
    prelude::{Biquad, Filter, SimulationState},
};
use core::time::Duration;

pub struct Bessel<T>
where
    T: Sample,
{
    cutoff_freq: T::Alpha,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> Bessel<T>
where
    T: Sample,
{
    fn base_parameters(cutoff_freq: T::Alpha, dt: Duration) -> (T::Alpha, T::Alpha, T::Alpha) {
        let ts = T::Alpha::from_duration(dt);
        let k = (T::Alpha::pi() * cutoff_freq * ts).tangent();
        let d: T::Alpha = 3.0.as_fp();
        let d = d.square_root();
        let a0 = d * k + k * k + 1.0.as_fp();

        (k, d, a0)
    }

    pub fn low_pass(cutoff_freq: T::Alpha, dt: Duration) -> Self {
        let (k, d, a0) = Self::base_parameters(cutoff_freq, dt);

        let b0 = k * k / a0;
        let b1 = b0 * 2.0.as_fp();
        let b2 = b0;
        let a1 = (k * k - 1.0.as_fp()) * 2.0.as_fp() / a0;
        let a2 = (-d * k + k * k + 1.0.as_fp()) / a0;

        Self {
            cutoff_freq,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn high_pass(cutoff_freq: T::Alpha, dt: Duration) -> Self {
        let (k, d, a0) = Self::base_parameters(cutoff_freq, dt);

        let num: T::Alpha = 1.0.as_fp();
        let b0 = num / a0;
        let b1 = b0 * (-2.0).as_fp();
        let b2 = b0;
        let a1 = (k * k - 1.0.as_fp()) * 2.0.as_fp() / a0;
        let a2 = (k * k - d * k + 1.0.as_fp()) / a0;

        Self {
            cutoff_freq,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn cutoff_freq(&self) -> T::Alpha {
        self.cutoff_freq
    }

    pub fn center_freq(&self) -> T::Alpha {
        self.cutoff_freq
    }

    pub fn bandwidth(&self) -> T::Alpha {
        let den: T::Alpha = 3.0.as_fp();

        self.cutoff_freq / den.square_root()
    }

    pub fn biquad_coefficients(&self) -> (T::Alpha, T::Alpha, T::Alpha, T::Alpha, T::Alpha) {
        self.biquad.coefficients()
    }
}

impl<T> Block for Bessel<T>
where
    T: Sample,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        self.biquad.block(input, sim_state)
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
    T: Sample,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
