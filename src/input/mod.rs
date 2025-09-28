use crate::signal::Signal;
use core::{ops::Shr, time::Duration};

pub mod impulse;
pub mod ramp;
pub mod sinusoid;
pub mod step;

pub trait Input {
    fn output(&mut self, dt: Duration) -> Signal;
}

pub trait AsInput: Sized + Input + 'static {
    fn as_input(&mut self) -> &mut dyn Input {
        self
    }
}

impl Shr<&mut dyn Input> for Duration {
    type Output = Signal;

    fn shr(self, rhs: &mut dyn Input) -> Self::Output {
        rhs.output(self)
    }
}
