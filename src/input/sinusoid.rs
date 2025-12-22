use crate::{block::Block, signal::Signal, time::TimeType};
use core::{f32::consts::PI, marker::PhantomData, ops::Add, time::Duration};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid<T, K>
where
    T: Float + Add<f64, Output = T>,
    K: TimeType,
{
    amplitude: T,
    period: Duration,
    phase: T,
    _marker: PhantomData<K>,
}

impl<T, K> Sinusoid<T, K>
where
    T: Float + Add<f64, Output = T>,
    K: TimeType,
{
    pub fn new(amplitude: T, period: Duration, phase: T) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Sinusoid<T, K>
where
    T: Float + Add<f64, Output = T>,
    K: TimeType,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            phase: T::zero(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Block for Sinusoid<T, K>
where
    T: Float + Add<f64, Output = T>,
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
        let value = self.amplitude * (self.phase + t / self.period.as_secs_f64());
        let value = libm::sin(value.to_f64().unwrap_or(0.0));
        let value = T::from(value).unwrap_or(T::zero());

        Signal {
            value,
            delta: input.delta,
        }
    }
}
