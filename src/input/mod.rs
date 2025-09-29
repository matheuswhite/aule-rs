use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::{ops::Mul, time::Duration};

pub mod impulse;
pub mod ramp;
pub mod sawtooth;
pub mod sinusoid;
pub mod square;
pub mod step;

pub trait Input {
    type Output;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output>;
    fn boxed(self) -> Box<dyn Input<Output = Self::Output>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn as_mut(&mut self) -> &mut dyn Input<Output = Self::Output>
    where
        Self: Sized + 'static,
    {
        self
    }
    fn as_ref(&self) -> &dyn Input<Output = Self::Output>
    where
        Self: Sized + 'static,
    {
        self
    }
}

impl<T> Mul<&mut dyn Input<Output = T>> for Duration {
    type Output = Signal<T>;

    fn mul(self, rhs: &mut dyn Input<Output = T>) -> Self::Output {
        rhs.output(self)
    }
}
