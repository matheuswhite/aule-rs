use crate::{block::Block, signal::Signal};
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

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let t = input.delta.sim_time().as_secs_f64();
        let value = self.amplitude * (self.phase + t / self.period.as_secs_f64());
        let value = libm::sin(value.to_f64().unwrap_or(0.0));
        let value = T::from(value).unwrap_or(T::zero());

        Signal {
            value,
            delta: input.delta,
        }
    }
}
