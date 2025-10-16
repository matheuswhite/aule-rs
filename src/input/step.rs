use core::marker::PhantomData;

use crate::{block::Block, signal::Signal, time::TimeType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step<T>
where
    T: TimeType,
{
    value: f32,
    _marker: PhantomData<T>,
}

impl<T> Step<T>
where
    T: TimeType,
{
    pub fn new(value: f32) -> Self {
        Step {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Step<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Step {
            value: 1.0,
            _marker: PhantomData,
        }
    }
}

impl<T> Block for Step<T>
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
        Signal {
            value: self.value,
            delta: input.delta,
        }
    }
}
