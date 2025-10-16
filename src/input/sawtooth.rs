use crate::{block::Block, signal::Signal, time::TimeType};
use core::{f32::consts::PI, marker::PhantomData, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth<T>
where
    T: TimeType,
{
    amplitude: f32,
    period: Duration,
    offset: f32,
    _marker: PhantomData<T>,
}

impl<T> Sawtooth<T>
where
    T: TimeType,
{
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Sawtooth<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            offset: 0.0,
            _marker: PhantomData,
        }
    }
}

impl<T> Block for Sawtooth<T>
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
        let period_secs = self.period.as_secs_f32();

        let value = (self.amplitude / period_secs) * (t % period_secs) + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
