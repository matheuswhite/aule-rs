use crate::{signal::Signal, time::TimeType};

pub trait Block {
    type Input;
    type Output;
    type TimeType: TimeType;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType>;
    fn last_output(&self) -> Option<Self::Output> {
        None
    }
    fn as_block(
        &mut self,
    ) -> &mut dyn Block<Input = Self::Input, Output = Self::Output, TimeType = Self::TimeType>
    where
        Self: Sized + 'static,
    {
        self
    }

    fn reset(&mut self) {}
}
