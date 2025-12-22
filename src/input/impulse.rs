use core::marker::PhantomData;

use num_traits::{Bounded, Zero};

use crate::{block::Block, signal::Signal, time::TimeType};

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T, K>
where
    T: Zero + Bounded,
    K: TimeType,
{
    value: Option<T>,
    _marker: PhantomData<K>,
}

impl<T, K> Impulse<T, K>
where
    T: Zero + Bounded,
    K: TimeType,
{
    pub fn new(value: T) -> Self {
        Impulse {
            value: Some(value),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Default for Impulse<T, K>
where
    T: Zero + Bounded,
    K: TimeType,
{
    fn default() -> Self {
        Self {
            value: Some(T::max_value()),
            _marker: PhantomData,
        }
    }
}

impl<T, K> Block for Impulse<T, K>
where
    T: Zero + Bounded,
    K: TimeType,
{
    type Input = ();
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let Some(value) = self.value.take() else {
            return Signal {
                value: T::zero(),
                delta: input.delta,
            };
        };

        self.value = None;
        Signal {
            value,
            delta: input.delta,
        }
    }
}
