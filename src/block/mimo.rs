use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::Mul;

pub trait MIMO {
    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal>;
    fn last_output(&self) -> Option<Vec<Signal>>;
    fn dimensions(&self) -> (usize, usize); // (inputs, outputs)
}

pub trait AsMIMO: Sized + MIMO + 'static {
    fn as_mimo(&mut self) -> &mut dyn MIMO {
        self
    }
}

impl<const I: usize> Mul<[Signal; I]> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: [Signal; I]) -> Self::Output {
        self.output(rhs.to_vec())
    }
}

impl Mul<&[Signal]> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &[Signal]) -> Self::Output {
        self.output(rhs.to_vec())
    }
}

impl Mul<Signal> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: Signal) -> Self::Output {
        self.output([rhs].to_vec())
    }
}

impl Mul<(Signal, Signal)> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: (Signal, Signal)) -> Self::Output {
        self.output([rhs.0, rhs.1].to_vec())
    }
}

impl Mul<(Signal, Signal, Signal)> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: (Signal, Signal, Signal)) -> Self::Output {
        self.output([rhs.0, rhs.1, rhs.2].to_vec())
    }
}

#[cfg(feature = "alloc")]
impl Mul<Vec<Signal>> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    fn mul(self, rhs: Vec<Signal>) -> Self::Output {
        self.output(rhs)
    }
}
