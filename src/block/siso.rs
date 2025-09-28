use crate::signal::Signal;
#[cfg(feature = "alloc")]
use core::ops::Mul;

pub trait SISO {
    fn output(&mut self, input: Signal) -> Signal;
    fn last_output(&self) -> Option<Signal>;
}

pub trait AsSISO: Sized + SISO + 'static {
    fn as_siso(&mut self) -> &mut dyn SISO {
        self
    }
}

impl Mul<Signal> for &mut dyn SISO {
    type Output = Signal;

    fn mul(self, input: Signal) -> Self::Output {
        self.output(input)
    }
}
