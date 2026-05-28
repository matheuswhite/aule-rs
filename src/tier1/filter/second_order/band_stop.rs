use crate::{
    block::Block,
    math::{
        float_point::{AsFloatPoint, FloatPoint},
        sample::Sample,
    },
    prelude::{Biquad, Filter, SimulationState},
};
use core::time::Duration;

pub struct BandStop<T>
where
    T: Sample,
{
    center_freq: T::Alpha,
    q_factor: T::Alpha,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> BandStop<T>
where
    T: Sample,
{
    pub fn new(center_freq: T::Alpha, q_factor: T::Alpha, dt: Duration) -> Self {
        let ts = T::Alpha::from_duration(dt);

        let k = (T::Alpha::pi() * center_freq * ts).tangent();
        let a0 = k / q_factor + k * k + 1.0.as_fp();

        let b0 = (k * k + 1.0.as_fp()) / a0;
        let b1 = (k * k - 1.0.as_fp()) * 2.0.as_fp() / a0;
        let b2 = b0;
        let a1 = b1;
        let a2 = (-k / q_factor + k * k + 1.0.as_fp()) / a0;

        Self {
            center_freq,
            q_factor,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn center_freq(&self) -> T::Alpha {
        self.center_freq
    }

    pub fn q_factor(&self) -> T::Alpha {
        self.q_factor
    }

    pub fn biquad_coefficients(&self) -> (T::Alpha, T::Alpha, T::Alpha, T::Alpha, T::Alpha) {
        self.biquad.coefficients()
    }
}

impl<T> Block for BandStop<T>
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

impl<T> Filter for BandStop<T>
where
    T: Sample,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
