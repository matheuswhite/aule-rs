use core::marker::PhantomData;

use crate::{block::Block, signal::Signal, time::TimeType};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ISE<T>
where
    T: TimeType,
{
    acc: f32,
    n: usize,
    _marker: PhantomData<T>,
}

impl<T> ISE<T>
where
    T: TimeType,
{
    pub fn value(&self) -> f32 {
        if self.n == 0 {
            0.0
        } else {
            self.acc / self.n as f32
        }
    }
}

impl<T> Block for ISE<T>
where
    T: TimeType,
{
    type Input = f32;
    type Output = f32;
    type TimeType = T;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        self.acc += input.value * input.value;
        self.n += 1;
        input
    }

    fn reset(&mut self) {
        self.acc = 0.0;
        self.n = 0;
    }
}
