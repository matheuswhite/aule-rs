use crate::{block::Block, prelude::SimulationState};
use core::{f32::consts::PI, ops::Add, time::Duration};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid<T>
where
    T: Float + Add<f64, Output = T>,
{
    amplitude: T,
    period: Duration,
    phase: T,
}

impl<T> Sinusoid<T>
where
    T: Float + Add<f64, Output = T>,
{
    pub fn new(amplitude: T, period: Duration, phase: T) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
        }
    }
}

impl<T> Default for Sinusoid<T>
where
    T: Float + Add<f64, Output = T>,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            phase: T::zero(),
        }
    }
}

impl<T> Block for Sinusoid<T>
where
    T: Float + Add<f64, Output = T>,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = sim_state.sim_time().as_secs_f64();
        let value = self.amplitude * (self.phase + t / self.period.as_secs_f64());
        let value = libm::sin(value.to_f64().unwrap_or(0.0));
        T::from(value).unwrap_or(T::zero())
    }
}
