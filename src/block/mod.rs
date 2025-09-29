use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub mod delay;
pub mod observer;
pub mod pid;
pub mod saturation;
pub mod smith_predictor;

pub trait Block {
    type Input;
    type Output;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output>;
    fn last_output(&self) -> Option<Signal<Self::Output>>;
    fn boxed(self) -> Box<dyn Block<Input = Self::Input, Output = Self::Output>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn as_ref(&self) -> &dyn Block<Input = Self::Input, Output = Self::Output>
    where
        Self: Sized + 'static,
    {
        self
    }
    fn as_mut(&mut self) -> &mut dyn Block<Input = Self::Input, Output = Self::Output>
    where
        Self: Sized + 'static,
    {
        self
    }
}
