use num_traits::{One, Zero};

use crate::{block::Block, signal::Signal, time::TimeType};
use core::{f32::consts::PI, marker::PhantomData, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Square<T, K>
where
    T: Zero + One + Copy,
    K: TimeType,
{
    amplitude: T,
    period: Duration,
    offset: T,
    _marker: PhantomData<K>,
}

impl<T, K> Square<T, K>
where
    T: Zero + One + Copy,
    K: TimeType,
{
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Square {
            amplitude,
            period,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Square<T, K>
where
    T: Zero + One + Copy,
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

impl<T, K> Block for Square<T, K>
where
    T: Zero + One + Copy,
    K: TimeType,
{
    type Input = ();
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let t = input.delta.sim_time().as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value = if (t % period_secs) < (period_secs / 2.0) {
            self.amplitude
        } else {
            T::zero()
        } + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
