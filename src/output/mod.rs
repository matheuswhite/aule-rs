use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub mod plotter;
pub mod printer;
pub mod writer;

pub trait Output<T> {
    fn show(&mut self, inputs: Signal<T>);
    fn boxed(self) -> Box<dyn Output<T>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn as_ref(&self) -> &dyn Output<T>
    where
        Self: Sized + 'static,
    {
        self
    }
    fn as_mut(&mut self) -> &mut dyn Output<T>
    where
        Self: Sized + 'static,
    {
        self
    }
}
