use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
pub mod good_hart;
pub mod iae;
pub mod ise;
pub mod itae;

pub trait Metric {
    type Input;

    fn update(&mut self, input: Signal<Self::Input>) -> Signal<Self::Input>;
    fn value(&self) -> f32;
    fn boxed(self) -> Box<dyn Metric<Input = Self::Input>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn as_mut(&mut self) -> &mut dyn Metric<Input = Self::Input>
    where
        Self: Sized + 'static,
    {
        self
    }
    fn as_ref(&self) -> &dyn Metric<Input = Self::Input>
    where
        Self: Sized + 'static,
    {
        self
    }
}
