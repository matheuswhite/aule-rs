use core::marker::PhantomData;

use crate::{block::Block, signal::Signal, time::TimeType};

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T>
where
    T: TimeType,
{
    value: Option<f32>,
    _marker: PhantomData<T>,
}

impl<T> Impulse<T>
where
    T: TimeType,
{
    pub fn new(value: f32) -> Self {
        Impulse {
            value: Some(value),
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Impulse<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Self {
            value: Some(f32::MAX),
            _marker: PhantomData,
        }
    }
}

impl<T> Block for Impulse<T>
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
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal {
                    value,
                    delta: input.delta,
                }
            }
            None => Signal {
                value: 0.0,
                delta: input.delta,
            }, // If no value is set, return 0.0
        }
    }
}
