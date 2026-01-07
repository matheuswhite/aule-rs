use crate::{block::Block, signal::Signal};
use core::{
    f32::consts::PI,
    ops::{Div, Mul},
    time::Duration,
};
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth<T>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
{
    amplitude: T,
    period: Duration,
    offset: T,
}

impl<T> Sawtooth<T>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
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
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
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
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
{
    type Input = ();
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let t = input.delta.sim_time().as_secs_f64();
        let period_secs = self.period.as_secs_f64();

        let value = (self.amplitude / period_secs) * (t % period_secs) + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
