use crate::{block::Block, signal::Signal, time::TimeType};
use core::marker::PhantomData;
use num_traits::One;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step<T, K>
where
    T: One + Copy,
    K: TimeType,
{
    value: T,
    _marker: PhantomData<K>,
}

impl<T, K> Step<T, K>
where
    T: One + Copy,
    K: TimeType,
{
    pub fn new(value: T) -> Self {
        Step {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Step<T, K>
where
    T: One + Copy,
    K: TimeType,
{
    fn default() -> Self {
        Step {
            value: T::one(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Block for Step<T, K>
where
    T: One + Copy,
    K: TimeType,
{
    type Input = ();
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        Signal {
            value: self.value,
            delta: input.delta,
        }
    }
}
