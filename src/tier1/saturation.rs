use core::marker::PhantomData;

use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;

#[derive(Debug, Clone)]
pub struct Saturation<T, D>
where
    T: Ord + Clone,
    D: TimeType,
{
    min: T,
    max: T,
    last_output: Option<T>,
    _marker: PhantomData<D>,
}

impl<T, D> Block for Saturation<T, D>
where
    T: Ord + Clone,
    D: TimeType,
{
    type Input = T;
    type Output = T;
    type TimeType = D;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let saturated_value = input.value.clamp(self.min.clone(), self.max.clone());
        let output = Signal {
            value: saturated_value,
            delta: input.delta,
        };
        self.last_output = Some(output.value.clone());
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone()
    }
}
