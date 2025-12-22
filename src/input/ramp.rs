use crate::{block::Block, signal::Signal, time::TimeType};
use core::{marker::PhantomData, ops::Mul};
use num_traits::One;

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp<T, K>
where
    T: One + Copy + Mul<f64, Output = T>,
    K: TimeType,
{
    value: T,
    _marker: PhantomData<K>,
}

impl<T, K> Ramp<T, K>
where
    T: One + Copy + Mul<f64, Output = T>,
    K: TimeType,
{
    pub fn new(value: T) -> Self {
        Ramp {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Ramp<T, K>
where
    T: One + Copy + Mul<f64, Output = T>,
    K: TimeType,
{
    fn default() -> Self {
        Self {
            value: T::one(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Block for Ramp<T, K>
where
    T: One + Copy + Mul<f64, Output = T>,
    K: TimeType,
{
    type Input = ();
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let value = self.value * input.delta.sim_time().as_secs_f64();
        Signal {
            value,
            delta: input.delta,
        }
    }
}
