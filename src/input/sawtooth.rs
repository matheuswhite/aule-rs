use crate::{block::Block, prelude::SimulationState};
use core::{
    f32::consts::PI,
    ops::{Div, Mul},
    time::Duration,
};
use num_traits::{Float, One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth<T>
where
    T: Float,
{
    amplitude: T,
    period: Duration,
    offset: T,
}

impl<T> Sawtooth<T>
where
    T: Float,
{
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
        }
    }
}

impl<T> Default for Sawtooth<T>
where
    T: Float,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            offset: T::zero(),
        }
    }
}

impl<T> Block for Sawtooth<T>
where
    T: Float,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = T::from(sim_state.sim_time().as_secs_f64()).unwrap();
        let period_secs = T::from(self.period.as_secs_f64()).unwrap();

        (self.amplitude / period_secs) * (t % period_secs) + self.offset
    }
}
