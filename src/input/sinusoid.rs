use crate::{block::Block, signal::Signal, time::TimeType};
use core::{f32::consts::PI, marker::PhantomData, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid<T>
where
    T: TimeType,
{
    amplitude: f32,
    period: Duration,
    phase: f32,
    _marker: PhantomData<T>,
}

impl<T> Sinusoid<T>
where
    T: TimeType,
{
    pub fn new(amplitude: f32, period: Duration, phase: f32) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Sinusoid<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            phase: 0.0,
            _marker: PhantomData,
        }
    }
}

impl<T> Block for Sinusoid<T>
where
    T: TimeType,
{
    type Input = ();
    type Output = f32;
    type TimeType = T;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let t = input.delta.sim_time().as_secs_f32();
        let value = self.amplitude * (t / self.period.as_secs_f32() + self.phase);
        #[cfg(feature = "std")]
        let value = value.sin();
        #[cfg(not(feature = "std"))]
        let value = libm::sin(value as f64) as f32;
        Signal {
            value,
            delta: input.delta,
        }
    }
}
