use crate::{block::Block, signal::Signal};
use core::{f32::consts::PI, time::Duration};
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Square<T>
where
    T: Zero + One + Copy,
{
    amplitude: T,
    period: Duration,
    offset: T,
}

impl<T> Square<T>
where
    T: Zero + One + Copy,
{
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Square {
            amplitude,
            period,
            offset,
        }
    }
}

impl<T> Default for Square<T>
where
    T: Zero + One + Copy,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            offset: T::zero(),
        }
    }
}

impl<T> Block for Square<T>
where
    T: Zero + One + Copy,
{
    type Input = ();
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
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
