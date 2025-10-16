use crate::{block::Block, signal::Signal, time::TimeType};
use core::{f32::consts::PI, marker::PhantomData, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Square<T>
where
    T: TimeType,
{
    amplitude: f32,
    period: Duration,
    offset: f32,
    _marker: PhantomData<T>,
}

impl<T> Square<T>
where
    T: TimeType,
{
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Square {
            amplitude,
            period,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Square<T>
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

impl<T> Block for Square<T>
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

        let value = if (t % period_secs) < (period_secs / 2.0) {
            self.amplitude
        } else {
            0.0
        } + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
