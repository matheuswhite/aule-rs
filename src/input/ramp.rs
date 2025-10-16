use core::marker::PhantomData;

use crate::{block::Block, signal::Signal, time::TimeType};

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp<T>
where
    T: TimeType,
{
    value: f32,
    _marker: PhantomData<T>,
}

impl<T> Ramp<T>
where
    T: TimeType,
{
    pub fn new(value: f32) -> Self {
        Ramp {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Ramp<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Self {
            value: 1.0,
            _marker: PhantomData,
        }
    }
}

impl<T> Block for Ramp<T>
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
        let value = self.value * input.delta.sim_time().as_secs_f32();
        Signal {
            value,
            delta: input.delta,
        }
    }
}
