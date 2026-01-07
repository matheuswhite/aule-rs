use crate::{block::Block, signal::Signal};
use core::ops::Mul;
use num_traits::One;

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    value: T,
}

impl<T> Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    pub fn new(value: T) -> Self {
        Ramp { value }
    }
}

impl<T> Default for Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    fn default() -> Self {
        Self { value: T::one() }
    }
}

impl<T> Block for Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    type Input = ();
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let value = self.value * input.delta.sim_time().as_secs_f64();
        Signal {
            value,
            delta: input.delta,
        }
    }
}
