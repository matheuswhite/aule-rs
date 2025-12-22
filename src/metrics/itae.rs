use crate::{block::Block, signal::Signal, time::TimeType};
use core::{
    marker::PhantomData,
    ops::{AddAssign, Div, Mul},
};
use num_traits::{Signed, Zero};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ITAE<T, K>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T>,
    K: TimeType,
{
    acc: T,
    n: usize,
    _marker: PhantomData<K>,
}

impl<T, K> ITAE<T, K>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T>,
    K: TimeType,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zero()
        } else {
            self.acc / self.n as f64
        }
    }
}

impl<T, K> Block for ITAE<T, K>
where
    T: Zero + Copy + Signed + Div<f64, Output = T> + AddAssign<T> + Mul<f64, Output = T>,
    K: TimeType,
{
    type Input = T;
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        self.n += 1;
        self.acc += input.value.abs() * self.n as f64;
        input
    }

    fn reset(&mut self) {
        self.acc = T::zero();
        self.n = 0;
    }
}
