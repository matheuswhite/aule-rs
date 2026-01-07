use crate::{block::Block, signal::Signal};
use core::ops::{AddAssign, Div, Mul};
use num_traits::{Signed, Zero};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ITAE<T>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T>,
{
    acc: T,
    n: usize,
}

impl<T> ITAE<T>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T>,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zero()
        } else {
            self.acc / self.n as f64
        }
    }
}

impl<T> Block for ITAE<T>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T> + Mul<f64, Output = T>,
{
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.n += 1;
        self.acc += input.value.abs() * self.n as f64;
        input
    }

    fn reset(&mut self) {
        self.acc = T::zero();
        self.n = 0;
    }
}
