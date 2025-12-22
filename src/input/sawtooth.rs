use crate::{block::Block, signal::Signal, time::TimeType};
use core::{
    f32::consts::PI,
    marker::PhantomData,
    ops::{Div, Mul},
    time::Duration,
};
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth<T, K>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
    K: TimeType,
{
    amplitude: T,
    period: Duration,
    offset: T,
    _marker: PhantomData<K>,
}

impl<T, K> Sawtooth<T, K>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
    K: TimeType,
{
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Sawtooth<T, K>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
    K: TimeType,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            offset: T::zero(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Block for Sawtooth<T, K>
where
    T: Zero + One + Copy + Div<f64, Output = T> + Mul<f64, Output = T>,
    K: TimeType,
{
    type Input = ();
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let t = input.delta.sim_time().as_secs_f64();
        let period_secs = self.period.as_secs_f64();

        let value = (self.amplitude / period_secs) * (t % period_secs) + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
